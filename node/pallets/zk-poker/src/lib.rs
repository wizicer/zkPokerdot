#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[macro_use]
extern crate uint;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

pub mod weights;
pub use weights::*;

pub mod common;
pub mod deserialization;
pub mod verify;

use frame_support::storage::bounded_vec::BoundedVec;
pub use pallet::*;
use sp_std::vec::Vec;

type PublicInputsDef<T> = BoundedVec<u8, <T as Config>::MaxPublicInputsLength>;
type ProofDef<T> = BoundedVec<u8, <T as Config>::MaxProofLength>;
type VerificationKeyDef<T> = BoundedVec<u8, <T as Config>::MaxVerificationKeyLength>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use crate::{
		common::prepare_verification_key,
		deserialization::{deserialize_public_inputs, Proof, VKey},
		verify::{
			verify, G1UncompressedBytes, G2UncompressedBytes, GProof, VerificationKey,
			SUPPORTED_CURVE, SUPPORTED_PROTOCOL,
		},
	};
	use bls12_381::Scalar;
	use core::cmp::Ordering;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sha2::{Digest, Sha256};

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type MaxPublicInputsLength: Get<u32>;

		/// The maximum length of the proof.
		#[pallet::constant]
		type MaxProofLength: Get<u32>;

		/// The maximum length of the verification key.
		#[pallet::constant]
		type MaxVerificationKeyLength: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		VerificationSetupCompleted,
		VerificationProofSet,
		VerificationSuccess {
			who: T::AccountId,
		},
		VerificationFailed,
		/// 游戏已经创建
		GameCreated(u32),
		/// 玩家加入游戏
		GamerJoined(u32),
		/// 玩家全部准备
		PlayerAllPrepared,
		GameCurPlayerId(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Public inputs mismatch
		PublicInputsMismatch,
		/// Public inputs vector is to long.
		TooLongPublicInputs,
		/// The verification key is to long.
		TooLongVerificationKey,
		/// The proof is too long.
		TooLongProof,
		/// The proof is too short.
		ProofIsEmpty,
		/// Verification key, not set.
		VerificationKeyIsNotSet,
		/// Malformed key
		MalformedVerificationKey,
		/// Malformed proof
		MalformedProof,
		/// Malformed public inputs
		MalformedPublicInputs,
		/// Curve is not supported
		NotSupportedCurve,
		/// Protocol is not supported
		NotSupportedProtocol,
		/// There was error during proof verification
		ProofVerificationError,
		/// Proof creation error
		ProofCreationError,
		/// Verification Key creation error
		VerificationKeyCreationError,
		/// 玩家人数已满
		GamerEnough,
		/// 请勿重复加入游戏
		PlayerAlreadyJoined,
		/// 游戏已经开始，不再洗牌
		GameStarted,
		WrongPlayerId,
	}

	/// Storing a public input.
	#[pallet::storage]
	pub type PublicInputStorage<T: Config> = StorageValue<_, PublicInputsDef<T>, ValueQuery>;

	/// Storing a proof.
	#[pallet::storage]
	pub type ProofStorage<T: Config> = StorageValue<_, ProofDef<T>, ValueQuery>;

	/// Storing a verification key.
	#[pallet::storage]
	pub type VerificationKeyStorage<T: Config> = StorageValue<_, VerificationKeyDef<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn my_game)]
	pub type Game<T: Config> = StorageMap<_, Blake2_128Concat, u32, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn my_game_players)]
	pub type GamePlayers<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, Vec<T::AccountId>, ValueQuery>;
	
	//gameId+sender 对照姓名
	#[pallet::storage]
	pub(super) type PlayerNames<T: Config> = StorageDoubleMap<
    	_,
    	Blake2_128Concat, u32,
    	Blake2_128Concat, T::AccountId,
    	Vec<u8>,
    	ValueQuery
	>;
	//gameId+sender 对照 玩家状态  0 未准备， 1 准备
	#[pallet::storage]
	pub(super) type PlayerStatus<T: Config> = StorageDoubleMap<
    	_,
    	Blake2_128Concat, u32,
    	Blake2_128Concat, T::AccountId,
    	u32,
    	ValueQuery
	>;




	/// 房间号对应洗完之后的牌
	#[pallet::storage]
	#[pallet::getter(fn my_game_decks)]
	pub type GameDecks<T: Config> = StorageMap<_, Blake2_128Concat, u32, Vec<u32>, ValueQuery>;

	/// 房间号当前的牌
	#[pallet::storage]
	#[pallet::getter(fn my_cur_cards)]
	pub type CurCards<T: Config> = StorageMap<_, Blake2_128Concat, u32, Vec<u32>, ValueQuery>;

	/// 房间号对应的当前玩家Id
	#[pallet::storage]
	#[pallet::getter(fn my_game_cur_player)]
	pub type GameCurPlayer<T: Config> = StorageMap<_, Blake2_128Concat, u32, T::AccountId>;

	/// 房间号对应的地主
	#[pallet::storage]
	#[pallet::getter(fn my_game_leader)]
	pub type GameLeader<T: Config> = StorageMap<_, Blake2_128Concat, u32, T::AccountId>;

	/// 房间号对应的游戏状态0 创建房间,1 所有玩家加入房间, 2 所有玩家准备 ，3已叫地主，4游戏结束  
	#[pallet::storage]
	#[pallet::getter(fn my_game_state)]
	pub type GameState<T: Config> = StorageMap<_, Blake2_128Concat, u32, u32, ValueQuery>;

	/// 房间号对应的三张底牌
	#[pallet::storage]
	#[pallet::getter(fn my_bottom_cards)]
	pub type BottomCards<T: Config> = StorageMap<_, Blake2_128Concat, u32, Vec<u32>, ValueQuery>;

	/// 玩家对应的手牌
	#[pallet::storage]
	#[pallet::getter(fn my_cards)]
	pub type PlayerCards<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u32>, ValueQuery>;
	
	//gameId+sender 对照 打出的牌  
	#[pallet::storage]
	pub(super) type HittedCards<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat, u32,
		Blake2_128Concat, T::AccountId,
		Vec<u32>,
		ValueQuery
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Store a verification key.
		#[pallet::weight(<T as Config>::WeightInfo::setup_verification_benchmark(vec_vk.len()))]
		pub fn setup_verification(_origin: OriginFor<T>, vec_vk: Vec<u8>) -> DispatchResult {
			let _vk = store_verification_key::<T>(vec_vk)?;
			Self::deposit_event(Event::<T>::VerificationSetupCompleted);
			Ok(())
		}

		/// Verify a proof.
		#[pallet::weight(<T as Config>::WeightInfo::verify_benchmark(vec_proof.len()))]
		pub fn verify(
			origin: OriginFor<T>,
			pub_input: Vec<u8>,
			vec_proof: Vec<u8>,
		) -> DispatchResult {
			let inputs = store_public_inputs::<T>(pub_input)?;
			let proof = store_proof::<T>(vec_proof)?;
			let vk = get_verification_key::<T>()?;
			// ensure!(vk.public_inputs_len == inputs.len() as u8,
			// Error::<T>::PublicInputsMismatch); let inputs = get_public_inputs::<T>()?;
			let sender = ensure_signed(origin)?;
			Self::deposit_event(Event::<T>::VerificationProofSet);

			match verify(vk, proof, inputs) {
				Ok(true) => {
					Self::deposit_event(Event::<T>::VerificationSuccess { who: sender });
					Ok(())
				},
				Ok(false) => {
					Self::deposit_event(Event::<T>::VerificationFailed);
					Ok(())
				},
				Err(_) => Err(Error::<T>::ProofVerificationError.into()),
			}
		}

		// 创建房间
		#[pallet::weight(0)]
		pub fn create_game(origin: OriginFor<T>, room: u32,playername:Vec<u8>) -> DispatchResultWithPostInfo {
			// Account calling this dispatchable.
			let sender = ensure_signed(origin)?;
			// 检查存储中是否已存在相同的seed
			ensure!(!Game::<T>::contains_key(&room), "Room already exists");
			let game_id: u32 = generate_room_number(room);
			Game::<T>::insert(&room, &game_id);
			// 游戏状态设为未开始
			GameState::<T>::insert(&game_id, 0);
			//创建初始玩家
			// 获取当前 key 对应的 Vec
			let accounts = GamePlayers::<T>::get(&game_id);
			// 检查玩家是否已存在于列表中
			if accounts.contains(&sender) {
				return Err(Error::<T>::PlayerAlreadyJoined.into())
			}

			// 向 vector 中添加一个新的 AccountID
			let mut new_accounts = accounts.clone();
			new_accounts.push(sender.clone());

			// 存入新的 Vector
			GamePlayers::<T>::insert(&game_id, new_accounts);
			PlayerNames::<T>::insert(&game_id,sender,playername);
			Self::deposit_event(Event::GameCreated(game_id));
			Ok(().into())
		}

		// 加入游戏
		#[pallet::weight(0)]
		pub fn join_game(origin: OriginFor<T>, game_name: u32,playername:Vec<u8>) -> DispatchResultWithPostInfo {
			// Account calling this dispatchable.
			let gamer = ensure_signed(origin)?;
			let game_id = Game::<T>::get(&game_name);
			// 获取当前 key 对应的 Vec
			let accounts = GamePlayers::<T>::get(&game_id);

			// 检查玩家数量是否超出限制
			if accounts.len() >= 3 {
				// 人数已满，游戏状态设为进行中
				GameState::<T>::insert(&game_id, 1);
				return Err(Error::<T>::GamerEnough.into())
			}

			// 检查玩家是否已存在于列表中
			if accounts.contains(&gamer) {
				return Err(Error::<T>::PlayerAlreadyJoined.into())
			}

			// 向 vector 中添加一个新的 AccountID
			let mut new_accounts = accounts.clone();
			new_accounts.push(gamer.clone());

			// 存入新的 Vector
			GamePlayers::<T>::insert(&game_id, new_accounts);
			PlayerNames::<T>::insert(&game_id,&gamer,playername);
			let num_players = accounts.len() as u32; // 玩家数量
			Self::deposit_event(Event::GamerJoined(num_players));

			Ok(().into())
		}

		// 洗牌和发牌
		#[pallet::weight(0)]
		pub fn shuffle(origin: OriginFor<T>, game_id: u32,cards: Vec<u32>) -> DispatchResultWithPostInfo {
			let gamer = ensure_signed(origin)?;
			GameDecks::<T>::insert(&game_id, cards.clone());
			// 更新存储中的状态
			PlayerStatus::<T>::insert(game_id, gamer, 1);
			//判断是否三位玩家都准备
			let mut count = 0;
        	for _ in PlayerStatus::<T>::iter_prefix(game_id) {
            	count += 1;
			}
			if count == 3 {
				if GameState::<T>::get(&game_id) == 2{
					return Err(Error::<T>::GameStarted.into())
				}
				GameState::<T>::insert(&game_id, 2);
				// 如果找到3条数据则可以开始发牌
				// 人数已满，游戏状态设为进行中
				// 按顺序发牌给三个玩家
				let mut player1_cards = Vec::new();
				let mut player2_cards = Vec::new();
				let mut player3_cards = Vec::new();

				// 留下的三张底牌
				let mut remaining_cards = Vec::new();

				// 分发牌给每位玩家
				for (index, &card) in cards.iter().enumerate() {
					match index {
						0..=16 => player1_cards.push(card),  // 第一位玩家的牌
						17..=33 => player2_cards.push(card), // 第二位玩家的牌
						34..=50 => player3_cards.push(card), // 第三位玩家的牌
						_ => remaining_cards.push(card),     // 底牌
					}
				}

				// 存储玩家的牌
				let accounts = GamePlayers::<T>::get(&game_id);
				PlayerCards::<T>::insert(&accounts[0], player1_cards);
				PlayerCards::<T>::insert(&accounts[1], player2_cards);
				PlayerCards::<T>::insert(&accounts[2], player3_cards);

				// 存储底牌
				BottomCards::<T>::insert(&game_id, remaining_cards);

				Self::deposit_event(Event::PlayerAllPrepared);
			}
        	
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn unmask(
			origin: OriginFor<T>,
			id: u32,
			unmask: Vec<u32>,
		) -> DispatchResultWithPostInfo {
			Ok(().into())
		}

		// 叫地主，先到先得
		#[pallet::weight(0)]
		pub fn call(origin: OriginFor<T>, game_id: u32) -> DispatchResultWithPostInfo {
			let gamer = ensure_signed(origin)?;
			ensure!(!GameLeader::<T>::contains_key(&game_id), "Room already has a leader");
			GameLeader::<T>::insert(&game_id, &gamer);
			// 设置地主为当前玩家
			GameCurPlayer::<T>::insert(&game_id, &gamer);
			let mut leadercards = PlayerCards::<T>::get(&gamer);
			let bottomcards = BottomCards::<T>::get(&game_id);
			//把底牌加入到地主牌当中
			for card in bottomcards{
				leadercards.push(card);
			}
			// 更新地主的牌
			PlayerCards::<T>::insert(&gamer, leadercards);
			// 更新游戏状态
			GameState::<T>::insert(&game_id, 3);
			Self::deposit_event(Event::GameCurPlayerId(gamer));
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn play(origin: OriginFor<T>, game_id: u32,cards: Vec<u32>) -> DispatchResultWithPostInfo {
			let gamer = ensure_signed(origin)?;
			let cur_player = GameCurPlayer::<T>::get(&game_id).ok_or(Error::<T>::WrongPlayerId)?;
			
			// 确保调用者是当前玩家
			ensure!(gamer == cur_player, Error::<T>::WrongPlayerId);
			
			// 更新HittedCards
			// HittedCards::<T>::mutate(game_id, &gamer, |hitted_cards| {
			// 	hitted_cards.extend(cards.clone());
				
			// });
			HittedCards::<T>::insert(game_id,&gamer,cards.clone());
			 // 获取玩家的手牌
			let mut player_cards = PlayerCards::<T>::get(&gamer);
			// 从PlayerCards中删除出的牌
			player_cards.retain(|card| !cards.contains(card));
			PlayerCards::<T>::insert(&gamer, player_cards);

			// 获取所有玩家的列表
			let accounts = GamePlayers::<T>::get(&game_id);
			
			// 找到当前玩家的位置并计算下一个玩家
			if let Some(index) = accounts.iter().position(|p| p == &gamer) {
				let next_index = (index + 1) % accounts.len();
				let next_player = &accounts[next_index];
		
				// 设置下一个玩家为当前玩家
				GameCurPlayer::<T>::insert(&game_id, next_player);
				Self::deposit_event(Event::GameCurPlayerId(next_player.clone()));
			} else {
				return Err(Error::<T>::WrongPlayerId.into());
			}
		
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn pass(origin: OriginFor<T>, game_id: u32) -> DispatchResultWithPostInfo {
			let gamer = ensure_signed(origin)?;
			let cur_player = GameCurPlayer::<T>::get(&game_id).ok_or(Error::<T>::WrongPlayerId)?;
			
			// 确保调用者是当前玩家
			ensure!(gamer == cur_player, Error::<T>::WrongPlayerId);
		
			// 获取所有玩家的列表
			let accounts = GamePlayers::<T>::get(&game_id);
			
			// 找到当前玩家的位置并计算下一个玩家
			if let Some(index) = accounts.iter().position(|p| p == &gamer) {
				let next_index = (index + 1) % accounts.len();
				let next_player = &accounts[next_index];
		
				// 设置下一个玩家为当前玩家
				GameCurPlayer::<T>::insert(&game_id, next_player);
				Self::deposit_event(Event::GameCurPlayerId(next_player.clone()));
			} else {
				return Err(Error::<T>::WrongPlayerId.into());
			}
		
			Ok(().into())
		}
		
	}

	#[allow(dead_code)]
	fn get_public_inputs<T: Config>() -> Result<Vec<Scalar>, sp_runtime::DispatchError> {
		let public_inputs = PublicInputStorage::<T>::get();
		let deserialized_public_inputs = deserialize_public_inputs(public_inputs.as_slice())
			.map_err(|_| Error::<T>::MalformedPublicInputs)?;
		Ok(deserialized_public_inputs)
	}

	fn store_public_inputs<T: Config>(
		pub_input: Vec<u8>,
	) -> Result<Vec<Scalar>, sp_runtime::DispatchError> {
		let public_inputs: PublicInputsDef<T> =
			pub_input.try_into().map_err(|_| Error::<T>::TooLongPublicInputs)?;
		let deserialized_public_inputs = deserialize_public_inputs(public_inputs.as_slice())
			.map_err(|_| Error::<T>::MalformedPublicInputs)?;
		PublicInputStorage::<T>::put(public_inputs);
		Ok(deserialized_public_inputs)
	}

	fn get_verification_key<T: Config>() -> Result<VerificationKey, sp_runtime::DispatchError> {
		let vk = VerificationKeyStorage::<T>::get();

		ensure!(!vk.is_empty(), Error::<T>::VerificationKeyIsNotSet);
		let deserialized_vk = VKey::from_json_u8_slice(vk.as_slice())
			.map_err(|_| Error::<T>::MalformedVerificationKey)?;
		let vk = prepare_verification_key(deserialized_vk)
			.map_err(|_| Error::<T>::VerificationKeyCreationError)?;
		Ok(vk)
	}

	fn store_verification_key<T: Config>(
		vec_vk: Vec<u8>,
	) -> Result<VKey, sp_runtime::DispatchError> {
		let vk: VerificationKeyDef<T> =
			vec_vk.try_into().map_err(|_| Error::<T>::TooLongVerificationKey)?;
		let deserialized_vk = VKey::from_json_u8_slice(vk.as_slice())
			.map_err(|_| Error::<T>::MalformedVerificationKey)?;
		ensure!(deserialized_vk.curve == SUPPORTED_CURVE.as_bytes(), Error::<T>::NotSupportedCurve);
		ensure!(
			deserialized_vk.protocol == SUPPORTED_PROTOCOL.as_bytes(),
			Error::<T>::NotSupportedProtocol
		);

		VerificationKeyStorage::<T>::put(vk);
		Ok(deserialized_vk)
	}

	fn store_proof<T: Config>(vec_proof: Vec<u8>) -> Result<GProof, sp_runtime::DispatchError> {
		ensure!(!vec_proof.is_empty(), Error::<T>::ProofIsEmpty);
		let proof: ProofDef<T> = vec_proof.try_into().map_err(|_| Error::<T>::TooLongProof)?;
		let deserialized_proof =
			Proof::from_json_u8_slice(proof.as_slice()).map_err(|_| Error::<T>::MalformedProof)?;
		ensure!(
			deserialized_proof.curve == SUPPORTED_CURVE.as_bytes(),
			Error::<T>::NotSupportedCurve
		);
		ensure!(
			deserialized_proof.protocol == SUPPORTED_PROTOCOL.as_bytes(),
			Error::<T>::NotSupportedProtocol
		);

		ProofStorage::<T>::put(proof);

		let proof = GProof::from_uncompressed(
			&G1UncompressedBytes::new(deserialized_proof.a[0], deserialized_proof.a[1]),
			&G2UncompressedBytes::new(
				deserialized_proof.b[0][0],
				deserialized_proof.b[0][1],
				deserialized_proof.b[1][0],
				deserialized_proof.b[1][1],
			),
			&G1UncompressedBytes::new(deserialized_proof.c[0], deserialized_proof.c[1]),
		)
		.map_err(|_| Error::<T>::ProofCreationError)?;

		Ok(proof)
	}

	fn generate_room_number(seed: u32) -> u32 {
		// 使用 sha256 哈希函数
		let mut hasher = Sha256::new();
		// 将 seed 转换为字节数组并输入到 hasher 中
		hasher.update(seed.to_be_bytes());
		let result = hasher.finalize();

		// 将哈希值转换为一个 u32 类型的房间号
		let hash_as_array = result.as_slice();
		let mut room_number: u32 = 0;
		for (i, &val) in hash_as_array.iter().enumerate() {
			room_number += (val as u32) << (i * 8);
		}

		room_number
	}

	#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
	enum Suit {
		Heart,
		Diamond,
		Club,
		Spade,
		Joker, // 用于表示大小王
	}

	#[derive(Debug, PartialEq, Eq, Clone, Copy)]
	enum Rank {
		Num(u32),
		Jack,
		Queen,
		King,
		Ace,
		Joker, // 用于表示大小王
	}

	impl PartialOrd for Rank {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			Some(self.cmp(other))
		}
	}

	impl Ord for Rank {
		fn cmp(&self, other: &Self) -> Ordering {
			match (self, other) {
				(Rank::Num(n1), Rank::Num(n2)) => n1.cmp(n2),
				(Rank::Num(_), _) => Ordering::Less,
				(_, Rank::Num(_)) => Ordering::Greater,
				(Rank::Jack, Rank::Jack) |
				(Rank::Queen, Rank::Queen) |
				(Rank::King, Rank::King) |
				(Rank::Ace, Rank::Ace) |
				(Rank::Joker, Rank::Joker) => Ordering::Equal,
				(Rank::Jack, _) => Ordering::Less,
				(_, Rank::Jack) => Ordering::Greater,
				(Rank::Queen, _) => Ordering::Less,
				(_, Rank::Queen) => Ordering::Greater,
				(Rank::King, _) => Ordering::Less,
				(_, Rank::King) => Ordering::Greater,
				(Rank::Ace, _) => Ordering::Less,
				(_, Rank::Ace) => Ordering::Greater,
				(Rank::Joker, Rank::Joker) => Ordering::Equal,
			}
		}
	}

	#[derive(Debug, PartialEq, Eq, Clone, Copy)]
	struct Card {
		rank: Rank,
		suit: Suit,
	}

	fn generate_deck() -> Vec<Card> {
		let mut deck = Vec::new();

		let suits = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];

		for suit in &suits {
			for rank in 2..11 {
				deck.push(Card { rank: Rank::Num(rank), suit: *suit });
			}
			deck.push(Card { rank: Rank::Jack, suit: *suit });
			deck.push(Card { rank: Rank::Queen, suit: *suit });
			deck.push(Card { rank: Rank::King, suit: *suit });
			deck.push(Card { rank: Rank::Ace, suit: *suit });
		}

		// 添加大小王
		deck.push(Card { rank: Rank::Joker, suit: Suit::Joker }); // 小王
		deck.push(Card { rank: Rank::Joker, suit: Suit::Joker }); // 大王

		deck
	}

	fn num_to_card(num: u32) -> Option<Card> {
		match num {
			1..=13 => Some(Card { rank: Rank::Num(num), suit: Suit::Heart }),
			14..=26 => Some(Card { rank: Rank::Num(num - 13), suit: Suit::Diamond }),
			27..=39 => Some(Card { rank: Rank::Num(num - 26), suit: Suit::Club }),
			40..=52 => Some(Card { rank: Rank::Num(num - 39), suit: Suit::Spade }),
			53 => Some(Card { rank: Rank::Joker, suit: Suit::Joker }), // 小王
			54 => Some(Card { rank: Rank::Joker, suit: Suit::Joker }), // 大王
			_ => None,
		}
	}
	#[derive(Debug)]
	enum HandType {
		Single(Card),
		Pair(Card, Card),
		Triple(Card, Card, Card),
		Bomb(Card, Card, Card, Card),
		// ... 其他牌型
	}

	impl PartialEq for HandType {
		fn eq(&self, other: &Self) -> bool {
			match (self, other) {
				(HandType::Single(card1), HandType::Single(card2)) => card1.rank == card2.rank,
				(HandType::Pair(card1_1, card1_2), HandType::Pair(card2_1, card2_2)) =>
					card1_1.rank == card2_1.rank && card1_2.rank == card2_2.rank,
				(
					HandType::Triple(card1_1, card1_2, card1_3),
					HandType::Triple(card2_1, card2_2, card2_3),
				) =>
					card1_1.rank == card2_1.rank &&
						card1_2.rank == card2_2.rank &&
						card1_3.rank == card2_3.rank,
				(
					HandType::Bomb(card1_1, card1_2, card1_3, card1_4),
					HandType::Bomb(card2_1, card2_2, card2_3, card2_4),
				) =>
					card1_1.rank == card2_1.rank &&
						card1_2.rank == card2_2.rank &&
						card1_3.rank == card2_3.rank &&
						card1_4.rank == card2_4.rank,
				// ... 其他牌型的比较规则
				_ => false,
			}
		}
	}

	impl PartialOrd for HandType {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			match (self, other) {
				(HandType::Single(card1), HandType::Single(card2)) =>
					card1.rank.partial_cmp(&card2.rank),
				(HandType::Pair(card1_1, _), HandType::Pair(card2_1, _)) =>
					card1_1.rank.partial_cmp(&card2_1.rank),
				(HandType::Triple(card1_1, _, _), HandType::Triple(card2_1, _, _)) =>
					card1_1.rank.partial_cmp(&card2_1.rank),
				(HandType::Bomb(card1_1, _, _, _), HandType::Bomb(card2_1, _, _, _)) =>
					card1_1.rank.partial_cmp(&card2_1.rank),
				// ... 其他牌型的比较规则
				_ => None,
			}
		}
	}
}
