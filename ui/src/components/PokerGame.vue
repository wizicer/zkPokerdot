<template>
  <div class="poker-ganme">
    <div class="table">
      <!-- 房间名字 -->
      <p class="room-name">Room Name:{{ roomName }}</p>
      <!-- 顶部三张牌 -->
      <div class="top-table">
        <el-card shadow="hover" :body-style="{ padding: '0px' }" v-for="(card, index) in topThree" :key="index">
          <img :src="`/src/images/${card.img}.png`" class="top-three" />
        </el-card>
      </div>
      <!-- 玩家手牌 -->
      <PlayerPokers :hands="playerMiddle.hands" />
      <!-- 左侧手牌 -->
      <div class="hands-card-left">
        <el-card shadow="hover" :body-style="{ padding: '0px' }">
          <img shadow="hover" src="../images/cardback.png" class="image" />
        </el-card>
        <!-- 左侧玩家背景 -->
        <div class="back"></div>
      </div>
      <div class="hands-card-right">
        <el-card shadow="hover" :body-style="{ padding: '0px' }">
          <img shadow="hover" src="../images/cardback.png" class="image" />
        </el-card>
        <!-- 右侧玩家背景 -->
        <div class="back"></div>
      </div>
    </div>
    <!-- 准备游戏 -->
    <div class="btn-group">
      <div class="ui-tip" v-show="gameState === 0">
        <el-button type="primary" @click="prepareGame">Prepare</el-button>
      </div>
      <!-- 叫地主和不叫的按钮 -->
      <div class="ui-tip" v-show="gameState === 2">
        <el-button type="success" @click="callGame">Call</el-button>
        <el-button type="info" @click="skipCallGame">skipCall</el-button>
      </div>
      <!-- 出牌和过的按钮 -->
      <div class="ui-tip" v-show="gameState === 3 && isTurn">
        <el-button type="primary" @click="playCards">Play</el-button>
        <el-button type="primary" @click="passTurn">Pass</el-button>
      </div>
    </div>
  </div>
  <div class="player-list">
    <el-avatar class="player-middle" v-show="playerMiddle.name !== ' '"> {{ playerMiddle.name }} </el-avatar>
    <el-avatar class="player-left" v-show="playerLeft.name !== ' '"> {{ playerLeft.name }} </el-avatar>
    <el-avatar class="player-right" v-show="playerRight.name !== ' '"> {{ playerRight.name }} </el-avatar>
    <div class="played-cards">
      <img v-for="(card, index) in playedCards" :src="`/src/images/${card.img}.png`" class="played-card" :key="index" />
    </div>
    <div class="played-cards-left">
      <img v-for="(card, index) in playedCardsLeft" :src="`/src/images/${card.img}.png`" class="played-card"
        :key="index" />
    </div>
    <div class="played-cards-right">
      <img v-for="(card, index) in playedCardsRight" :src="`/src/images/${card.img}.png`" class="played-card"
        :key="index" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import PlayerPokers from './PokerCard.vue';
import { ElLoading } from 'element-plus';
import type { PokerCard } from '../constant/poker';
import type { Player } from '../constant/roomState';
import type { Ref } from 'vue';
import { useRoute } from 'vue-router';
import { Room } from '../constant/roomState';
import { dealCards } from '../constant/poker';
import { initializeWeb3, loading, localRun, initPokerGame, readyGame, robLandlord,pass,play } from '../services/palletConnet';


const gameState = ref(0);
const route = useRoute();
const roomName = route.query.roomName as string;
const gameId = route.query.gameId as string;

//顶部三张牌
let topThree: Ref<PokerCard[]> = ref([{
  id: 1,
  suit: ' ',
  rank: ' ',
  img: 'cardback',
  isSelected: false
},
{
  id: 1,
  suit: ' ',
  rank: ' ',
  img: 'cardback',
  isSelected: false
},
{
  id: 1,
  suit: ' ',
  rank: ' ',
  img: 'cardback',
  isSelected: false
},
]
);
// 打出的牌
let playedCards: Ref<PokerCard[]> = ref([]);
//左侧显示牌
let playedCardsLeft: Ref<PokerCard[]> = ref([]);
//右侧显示牌
let playedCardsRight: Ref<PokerCard[]> = ref([]);
//玩家显示
let playerMiddle: Ref<Player> = ref({
  id:'',
  name: route.query.playerName as string,
  hands: [],
  isLandlord: false,
  isReady: false,
  hitedHands: []
});
let playerLeft: Ref<Player> = ref({
  id:'',
  name: ' ',
  hands: [],
  isLandlord: false,
  isReady: false,
  hitedHands: []
});
let playerRight: Ref<Player> = ref({
  id:'',
  name: ' ',
  hands: [],
  isLandlord: false,
  isReady: false,
  hitedHands: []
});
const currentPlayerIndex = ref(-1);
const isTurn = ref(false);
// 当 localStorage 变化时调用的函数
function handleStorageChange(event: StorageEvent) {
  // 检查 event.key 和 event.newValue 来响应特定的变化
  if (event.key === roomName) {
    console.log('localStorage changed:', event.newValue);
    // 这里你可以根据变化更新组件的状态
  }
  //处理人员加入逻辑
  const roomState = Room.getRoomState(roomName); //获取信息
  console.log('localStorage 变化', roomState);
  if (playerMiddle.value.name === 'Alice' && roomState.roomState === 'waiting') {
    if (roomState.players.length == 2) {
      playerRight.value.name = roomState.players[1].name;
    }
    if (roomState.players.length == 3) {
      playerLeft.value.name = roomState.players[2].name;
    }
  }
  if (playerMiddle.value.name === 'Bob' && roomState.roomState === 'waiting') {
    if (roomState.players.length == 2) {
      playerLeft.value.name = roomState.players[1].name;
    }
    if (roomState.players.length == 3 && roomState.roomState === 'waiting') {
      playerRight.value.name = roomState.players[2].name;
    }
  }
  //是否要判断游戏开始，考虑一下
  if (roomState.roomState === 'inProgress' || roomState.roomState === 'called')//游戏开始 更新牌组
  {
    if (playerMiddle.value.name === 'Alice') {
      playerMiddle.value.hands = roomState.players[0].hands;
    }
    if (playerMiddle.value.name === 'Bob') {
      playerMiddle.value.hands = roomState.players[1].hands;
    }
    if (playerMiddle.value.name === 'Carol') {
      playerMiddle.value.hands = roomState.players[2].hands;
    }
    if (roomState.roomState === 'called') { topThree.value = roomState.remainingCards; }
    playedCardsLeft.value = roomState.players.find(p => p.name === playerLeft.value.name)?.hitedHands ?? [];
    playedCardsRight.value = roomState.players.find(p => p.name === playerRight.value.name)?.hitedHands ?? [];
    currentPlayerIndex.value = roomState.currentPlayerIndex;
    gameState.value = 2;
  }
}

onMounted(async () => {
  if (localRun) {
    window.addEventListener('storage', handleStorageChange);
    const roomState = Room.getRoomState(roomName);
    console.log(roomState.players);
    let startIndex = -1;
    for (let i = 0; i < roomState.players.length; i++) {
      if (playerMiddle.value.name === roomState.players[i].name) {
        startIndex = i;
        break;
      }
    }
    if (startIndex !== -1) {
      for (let j = 0; j < roomState.players.length; j++) {
        if (j === 0) {
          continue;
        }
        let currentIndex = (startIndex + j) % roomState.players.length;
        if (j === 1) {
          playerRight.value.name = roomState.players[currentIndex].name;
        }
        if (j === 2) {
          playerLeft.value.name = roomState.players[currentIndex].name;
        }
      }
    } else {
      console.log('Player not found');
    }
  }
  else {
    console.log('正常运行');
    initPokerGame(gameId, playerMiddle, playerLeft, playerRight, topThree, gameState, isTurn,playedCards,playedCardsLeft,playedCardsRight);
  }
});

onUnmounted(() => {
  if (localRun) {
    window.removeEventListener('storage', handleStorageChange);
  }
  else {
    console.log('onUnmounted');
  }
});


const prepareGame = async () => {
  console.log("准备游戏");//准备游戏
  if (localRun) {
    await loading(9000, 'Preparing to shuffle');
    await initializeWeb3();

    Room.updatePlayerStatus(roomName, playerMiddle.value.name, true);
    if (Room.areThreePlayersReady(roomName) === true)//判断三个玩家都已经准备
    {
      currentPlayerIndex.value = 0;
      Room.setCurrentPlayerIndex(roomName, currentPlayerIndex.value);//设置应该哪个玩家出牌
      Room.setRoomState(roomName, "inProgress");//设置房间状态进行中
      const { playersCards, remainingCards } = dealCards();//获取玩家牌和顶部牌
      Room.distributeCards(roomName, playersCards, remainingCards);//分给localstroge
      const roomState = Room.getRoomState(roomName);
      if (playerMiddle.value.name === 'Alice') {
        playerMiddle.value.hands = roomState.players[0].hands;
      }
      if (playerMiddle.value.name === 'Bob') {
        playerMiddle.value.hands = roomState.players[1].hands;
      }
      if (playerMiddle.value.name === 'Carol') {
        playerMiddle.value.hands = roomState.players[2].hands;
      }
      console.log(roomState);
    }
  }
  else {
    gameState.value = 1;
    const loading = ElLoading.service({ lock: true, text: 'preparing', background: 'rgba(0, 0, 0, 0.7)', })
    await readyGame(gameId);
    loading.close();
  }
}

const playCards = async () => {
  console.log("出牌");
  if (localRun) {
    await loading(1500, 'Playing');
    await initializeWeb3();
    const selectedCards = playerMiddle.value.hands.filter(card => card.isSelected);
    // 筛选出未选择的牌
    const remainingCards = playerMiddle.value.hands.filter(card => !card.isSelected);
    // 更新玩家手牌
    playerMiddle.value.hands = remainingCards;
    // 更新 localStorage 中的游戏状态
    Room.updatePlayerStatus(roomName, playerMiddle.value.name, undefined, true, remainingCards);
    Room.setHitedHands(roomName, playerMiddle.value.name, selectedCards);
    playedCards.value = selectedCards;
    currentPlayerIndex.value += 1;
    Room.setCurrentPlayerIndex(roomName, currentPlayerIndex.value);
  }
  else {
    const loading = ElLoading.service({ lock: true, text: 'playing', background: 'rgba(0, 0, 0, 0.7)', })
    const selectedCards = playerMiddle.value.hands.filter(card => card.isSelected);
    await play(gameId,isTurn,selectedCards);
    loading.close();
  }
}
const passTurn = async () => {
  console.log("过");//过
  if (localRun) {
    await initializeWeb3();
    playedCards.value = [];
    Room.setHitedHands(roomName, playerMiddle.value.name, []);//清空显示
    currentPlayerIndex.value += 1;
    Room.setCurrentPlayerIndex(roomName, currentPlayerIndex.value);
  }
  else {
    const loading = ElLoading.service({ lock: true, text: 'passing', background: 'rgba(0, 0, 0, 0.7)', })
    await pass(gameId,isTurn);
    loading.close();
  }
}
const callGame = async () => {
  console.log("叫地主");//叫地主
  if (localRun) {
    await initializeWeb3();
    gameState.value = 3;
    const roomState = Room.getRoomState(roomName);
    Room.updatePlayerStatus(roomName, playerMiddle.value.name, undefined, true, playerMiddle.value.hands.concat(roomState.remainingCards));
    Room.setRoomState(roomName, "called");
    topThree.value = roomState.remainingCards;
    playerMiddle.value.hands = Room.getRoomState(roomName).players[0].hands;
    console.log(roomState);
  }
  else {
    const loading = ElLoading.service({ lock: true, text: 'calling', background: 'rgba(0, 0, 0, 0.7)', })
    await robLandlord(gameId,gameState,isTurn);
    loading.close();
  }
}
const skipCallGame = () => {
  console.log("不叫");//不叫
}
</script>

<style scoped>
.poker-ganme {
  height: 100vh;
  /* 全屏高度 */
  display: flex;
  flex-direction: column;

  .table {
    position: relative;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 20;

    .room-name {
      position: absolute;
      top: 0;
      left: 50%;
      transform: translate(-50%, 0);
      display: flex;
      height: 50px;
      margin: 10px;
      text-align: center;
      border-radius: 4px;
      color: var(--el-color-primary);
    }

    .top-table {
      position: absolute;
      top: +3vw;
      left: 50%;
      transform: translate(-50%, 0);
      display: flex;

      .top-three {
        width: 68px;
        height: 88px;
      }
    }




    .hands-card-left {
      position: absolute;
      left: 20vw;
      top: 40vh;
      width: 9vh;
      height: 12vh;
      padding: 0;
    }

    .hands-card-right {
      position: absolute;
      right: 20vw;
      top: 40vh;
      width: 9vh;
      height: 12vh;
      padding: 0;
    }
  }
}

.player-list {
  position: absolute;
  width: 100%;
  height: 100%;
  display: flex;
  top: 0;

  .player-left {
    position: absolute;
    left: 16vw;
    top: 45vh;
  }

  .player-right {
    position: absolute;
    right: 17vw;
    top: 45vh;
  }

  .player-middle {
    position: absolute;
    left: 35vw;
    top: 90vh;
  }

  .played-cards {
    position: absolute;
    top: 50vh;
    left: 45vw;
    display: flex;
    width: 68px;
    height: 88px;
  }

  .played-cards-left {
    position: absolute;
    top: 30vh;
    left: 30vw;
    display: flex;
    width: 68px;
    height: 88px;
  }

  .played-cards-right {
    position: absolute;
    top: 30vh;
    left: 60vw;
    display: flex;
    width: 68px;
    height: 88px;
  }
}

.btn-group {
  width: 50vw;
  position: absolute;
  left: 50%;
  top: 58%;
  z-index: 30;
}
</style>../services/palletConnet
