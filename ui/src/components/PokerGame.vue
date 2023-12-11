<template>
  <div class="poker-ganme">
    <div class="table">
      <div class="top-table">顶部三张牌</div>
      <div class="player-pokers">玩家手牌</div>
      <div class="hands-card-left">左侧玩家手牌
        <div class="back">背景图</div>
      </div>
      <div class="hands-card-right">右侧玩家手牌
        <div class="back">背景图</div>
      </div>
    </div>
    <div class="player-list">
      <div class="player1">玩家一</div>
      <div class="player2">玩家二</div>
      <div class="player3">玩家三</div>
    </div>
    <!-- 叫地主和不叫的按钮 -->
    <div class="btn-group">
      <div class="ui-tip" v-show="!gamePrepared">
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
</template>

<script setup lang="ts">
import { ref } from 'vue';
import PokerCard from './PokerCard.vue';

const gamePrepared = ref(false);
const gameStarted = ref(true);
const gamePlayed = ref(true);
const prepareGame = () => {
  console.log("准备游戏");//准备游戏
  gamePrepared.value = true;
}
const playCards = () => {
  gamePlayed.value = true;
  console.log("出牌");//出牌
}
const passTurn = () => {
  console.log("过");//过
}
const callGame = () => {
  console.log("叫地主");//叫地主
  gameStarted.value = true;
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
}

.btn-group {
  width: 50vw;
  position: absolute;
  left: 50%;
  top: 58%;
  z-index: 30;
}
</style>
