type Suit = "spades" | "hearts" | "diamonds" | "clubs" | "red" | "black";
type Rank = "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "J" | "Q" | "K" | "A" | "JOKER";
type SortKey = string;

export interface PokerCard {
  id: string;
  suit: Suit;
  sortKey: SortKey;
  isSelected: boolean;
}

const NUMS: Rank[] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
const SUITS: Suit[] = ["spades", "hearts", "diamonds", "clubs"];
let POKERS: PokerCard[] = [];

NUMS.forEach((num) => {
  SUITS.forEach((suit) => {
    POKERS.push({
      id: num + '_of_' + suit,
      suit: suit,
      sortKey: num,
      isSelected: false,
    });
  });
});

// Add jokers
POKERS = POKERS.concat([
  {
    id: "red_joker",
    suit: "red",
    sortKey: "red",
    isSelected: false,
  },
  {
    id: "black_joker",
    suit: "black",
    sortKey: "black",
    isSelected: false,
  },
]);

function createPokerCard(id:string):PokerCard{
  let suit: Suit;
  let sortKey: SortKey;
  const isSelected = false; // 默认值，可以根据需要调整
  // 示例逻辑：根据id确定suit和sortKey
  // 这里的逻辑应该根据你的具体需求来定制
  if (id === "特定ID1") {
      suit = 'spades';
      sortKey = '2'; 
  } else if (id === "特定ID2") {
      suit = 'spades';
      sortKey = "3"; 
  } else {
      // 默认值或错误处理
      suit = 'spades';
      sortKey = '2';
  }

  return { id, suit, sortKey, isSelected };
}

export default POKERS;
