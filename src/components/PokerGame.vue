<template>
  <el-container class="poker-game-container">
    <!-- 游戏区域 -->
    <el-row class="game-area">
      <!-- 玩家区域 -->
      <el-col :span="8" class="player-sidebar">
        <!-- 玩家二的区域 -->
      </el-col>
      <el-col :span="8" class="player-main">
        <!-- 玩家一的牌 -->
        <div class="player-cards">
          <poker-card v-for="card in playerOneCards" :key="card" :card="card" />
        </div>
        <!-- 控制按钮 -->
        <div class="game-controls">
          <el-button type="primary" @click="prepareGame">准备</el-button>
          <el-button type="warning" @click="callGame">叫地主</el-button>
          <el-button type="primary" @click="playCards">出牌</el-button>
          <el-button type="primary" @click="passTurn">过</el-button>
        </div>
      </el-col>
      <el-col :span="8" class="player-sidebar">
        <!-- 玩家三的区域 -->
      </el-col>
    </el-row>
  </el-container>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import PokerCard from './PokerCard.vue';

const playerOneCards = ref(['J', 'Q', 'K', 'A']);
const prepareGame = () => {
  console.log("准备游戏");//准备游戏
}
const playCards = () => {
  console.log("出牌");//出牌
}
const passTurn = () => {
  console.log("过");//出牌
}
const callGame = () => {
  console.log("叫地主");//叫地主
}
</script>

<style scoped></style>
