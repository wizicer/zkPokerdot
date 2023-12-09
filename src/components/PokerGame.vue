<template>
    <el-container>
      <!-- 玩家区域 -->
      <el-row class="player-row" v-for="player in players" :key="player.id">
        <el-col :span="24">
          <div class="player">
            <div class="player-name">{{ player.name }}</div>
            <div class="player-cards">
              <!-- 展示每位玩家的扑克牌 -->
              <el-card class="poker-card" v-for="card in player.cards" :key="card">
                {{ card }}
              </el-card>
            </div>
          </div>
        </el-col>
      </el-row>
    </el-container>
  </template>
  
  <script setup lang="ts">
  import { ref } from 'vue';
  
  // 示例数据
  const players = ref([
    { id: 1, name: '玩家1', cards: ['♠️A', '♥️K', '♦️Q'] },
    { id: 2, name: '玩家2', cards: ['♣️J', '♠️10', '♥️9'] },
    { id: 3, name: '玩家3', cards: ['♦️8', '♣️7', '♠️6'] }
  ]);
  </script>
  
  <style scoped>
  .player-row {
    margin-bottom: 20px;
  }
  
  .player {
    text-align: center;
  }
  
  .player-name {
    margin-bottom: 10px;
    font-weight: bold;
  }
  
  .player-cards .poker-card {
    display: inline-block;
    margin-right: 10px;
  }
  </style>
  