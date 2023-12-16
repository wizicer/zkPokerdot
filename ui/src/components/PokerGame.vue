<template>
  <div class="poker-ganme">
    <div class="table">
      <!-- 房间名字 -->
      <p class="room-name">房间名:{{roomName}}</p>
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
          v-for="(poker, index) in player1.hands" :key="poker.id" :style="{ left: index * 20 + 'px' }"
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
        <el-button type="primary" @click="prepareGame">准备</el-button>
      </div>
      <div class="ui-tip" v-show="!gameStarted">
        <el-button type="success" @click="callGame">叫地主</el-button>
        <el-button type="info" @click="skipCallGame">不叫</el-button>
      </div>
      <div class="ui-tip" v-show="!gamePlayed">
        <el-button type="primary" @click="playCards">出牌</el-button>
        <el-button type="primary" @click="passTurn">过</el-button>
      </div>
    </div>
  </div>
  <div class="player-list">
    <el-avatar class="player-middle" v-show="!gamePrepared"> Alice </el-avatar>
    <el-avatar class="player-left" v-show="!gamePrepared"> Bob </el-avatar>
    <el-avatar class="player-right" v-show="!gamePrepared"> Carol </el-avatar>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
//import PokerCard from './PokerCard.vue';
import POKERS from '../constant/poker';
import type { PokerCard } from '../constant/poker';
import type { Ref } from 'vue';
import { useRoute } from 'vue-router';

const gamePrepared = ref(true);//已经准备
const gameStarted = ref(true);//已经开始
const gamePlayed = ref(true);//已经玩了
const route = useRoute();
const roomName = route.query.roomName;

interface Player {
  name: string,//名字
  hands: PokerCard[],//手牌
  isLandlord: boolean//地主
}

let player1: Ref<Player> = ref({
  name: 'Alice',
  hands: [],
  isLandlord: false
});

function getRandomCards(cards: PokerCard[], count: number): PokerCard[] {
  let shuffled = cards.slice();
  for (let i = shuffled.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]];
  }
  return shuffled.slice(0, count);
}

const prepareGame = () => {
  console.log("准备游戏");//准备游戏
  gamePrepared.value = false;
  player1.value.hands = getRandomCards(POKERS, 17);
  player1.value.hands.sort((a, b) => b.id - a.id);
  console.log(player1.value.hands);
  gameStarted.value = false;
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
