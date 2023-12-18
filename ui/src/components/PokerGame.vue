<template>
  <div class="poker-ganme">
    <div class="table">
      <!-- 房间名字 -->
      <p class="room-name">Room Name:{{ roomName }}</p>
      <!-- 顶部三张牌 -->
      <div class="top-table">
        <el-card shadow="hover" :body-style="{ padding: '0px' }">
          <img src="../images/cardback.png" class="image" />
        </el-card>
        <el-card shadow="hover" :body-style="{ padding: '0px' }">
          <img shadow="hover" src="../images/cardback.png" class="image" />
        </el-card>
        <el-card shadow="hover" :body-style="{ padding: '0px' }">
          <img src="../images/cardback.png" class="image" />
        </el-card>
      </div>
      <!-- 玩家手牌 -->
      <div class="player-pokers">
        <el-card shadow="hover" :body-style="{ padding: '0px' }" class="player-card"
          v-for="(poker, index) in playerMiddle.hands" :key="poker.id" :style="{ left: index * 20 + 'px' }"
          :class="{ 'is-selected': poker.isSelected }">
          <img :src="`/src/images/${poker.img}.png`" class="image" :style="{ width: '120px', height: 'auto' }"
            @click="selectCard(poker)" />
        </el-card>
      </div>
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
    <!-- 叫地主和不叫的按钮 -->
    <div class="btn-group">
      <div class="ui-tip" v-show="gamePrepared">
        <el-button type="primary" @click="prepareGame">Prepare</el-button>
      </div>
      <div class="ui-tip" v-show="!gameStarted && isTurn">
        <el-button type="success" @click="callGame">Call</el-button>
        <el-button type="info" @click="skipCallGame">skipCall</el-button>
      </div>
      <div class="ui-tip" v-show="!gamePlayed">
        <el-button type="primary" @click="playCards">Play</el-button>
        <el-button type="primary" @click="passTurn">Pass</el-button>
      </div>
    </div>
  </div>
  <div class="player-list">
    <el-avatar class="player-middle" v-show="playerMiddle.name !== ' '"> {{ playerMiddle.name }} </el-avatar>
    <el-avatar class="player-left" v-show="playerLeft.name !== ' '"> {{ playerLeft.name }} </el-avatar>
    <el-avatar class="player-right" v-show="playerRight.name !== ' '"> {{ playerRight.name }} </el-avatar>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted,computed  } from 'vue';
//import PokerCard from './PokerCard.vue';
import type { PokerCard } from '../constant/poker';
import type { Player, RoomState } from '../constant/roomState';
import type { Ref } from 'vue';
import { useRoute } from 'vue-router';
import { Room } from '../constant/roomState';
import { dealCards } from '../constant/poker';

const gamePrepared = ref(true);//已经准备
const gameStarted = ref(true);//已经开始
const gamePlayed = ref(true);//已经玩了
const route = useRoute();
const roomName = route.query.roomName as string;
const local = ref(true)

let playerMiddle: Ref<Player> = ref({
  name: ' ',
  hands: [],
  isLandlord: false,
  isReady: false
});
let playerLeft: Ref<Player> = ref({
  name: ' ',
  hands: [],
  isLandlord: false,
  isReady: false
});
let playerRight: Ref<Player> = ref({
  name: ' ',
  hands: [],
  isLandlord: false,
  isReady: false
});
const currentPlayerIndex = ref(0);
const isTurn = computed(() => {
  if(currentPlayerIndex.value === 1 && playerMiddle.value.name === 'Alice')
  {
    return true;
  }
  if(currentPlayerIndex.value === 2 && playerMiddle.value.name === 'Bob')
  {
    return true;
  }
  if(currentPlayerIndex.value === 3 && playerMiddle.value.name === 'Carol')
  {
    return true;
  }
  return false;
});
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
  if(roomState.roomState === 'inProgress')//游戏开始 更新牌组
  {
    
    if(playerMiddle.value.name === 'Alice')
    {
      playerMiddle.value.hands = roomState.players[0].hands;
    }
    if(playerMiddle.value.name === 'Bob')
    {
      playerMiddle.value.hands = roomState.players[1].hands;
    }
    if(playerMiddle.value.name === 'Carol')
    {
      playerMiddle.value.hands = roomState.players[2].hands;
    }
  }
}

onMounted(() => {
  if (local.value) {
    window.addEventListener('storage', handleStorageChange);
    const roomState = Room.getRoomState(roomName);
    console.log(roomState.players);
    playerMiddle.value.name = roomState.players[roomState.players.length - 1].name;
    if (roomState.players[roomState.players.length - 1].name === 'Bob') {
      playerLeft.value.name = 'Alice';
    }
    if (roomState.players[roomState.players.length - 1].name === 'Carol') {
      playerRight.value.name = 'Alice';
      playerLeft.value.name = 'Bob';
    }
  }
});

onUnmounted(() => {
  if (local.value) {
    window.removeEventListener('storage', handleStorageChange);
  }
});


const prepareGame = () => {
  console.log("准备游戏");//准备游戏
  gamePrepared.value = false;
  //playerMiddle.value.hands.sort((a, b) => b.id - a.id);
  gameStarted.value = false;
  Room.updatePlayerStatus(roomName,playerMiddle.value.name,true);
  if(Room.areThreePlayersReady(roomName) === true)//判断三个玩家都已经准备
  {
    Room.setCurrentPlayerIndex(roomName,1);//设置应该哪个玩家出牌
    currentPlayerIndex.value = 1;
    Room.setRoomState(roomName,"inProgress");//设置房间状态进行中
    const { playersCards, remainingCards } = dealCards();//获取玩家牌和顶部牌
    Room.distributeCards(roomName,playersCards,remainingCards);//分给localstroge
    const roomState = Room.getRoomState(roomName);
    if(playerMiddle.value.name === 'Alice')
    {
      playerMiddle.value.hands = roomState.players[0].hands;
    }
    if(playerMiddle.value.name === 'Bob')
    {
      playerMiddle.value.hands = roomState.players[1].hands;
    }
    if(playerMiddle.value.name === 'Carol')
    {
      playerMiddle.value.hands = roomState.players[2].hands;
    }
    console.log(roomState);
  }
}
const playCards = () => {
  console.log("出牌");//出牌
}
const passTurn = () => {
  console.log("过");//过
}
const callGame = () => {
  console.log("叫地主");//叫地主
  gamePlayed.value = false;
  gameStarted.value = true;
}
const skipCallGame = () => {
  console.log("不叫");//不叫
}
const selectCard = (poker: PokerCard) => {
  poker.isSelected = !poker.isSelected
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
    }

    .player-pokers {
      height: 31.5vh;
      position: absolute;
      left: 50%;
      bottom: 0.5vw;
      transform: translate(-50%, 0);
      display: flex;
      width: calc(16 * 20px + 120px);

      .player-card {
        position: absolute;
      }

      .player-card.is-selected {
        top: -2vw;
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
}

.btn-group {
  width: 50vw;
  position: absolute;
  left: 50%;
  top: 58%;
  z-index: 30;
}
</style>
