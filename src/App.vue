<template>
  <div class="room-management">
    <h1>房间管理</h1>
    
    <div class="button-container">
      <!-- 加入房间按钮 -->
      <button class="join-room-button" @click="joinRoom">加入房间</button>

      <!-- 创建房间按钮 -->
      <button class="create-room-button" @click="createRoom">创建房间</button>
    </div>
  </div>
</template>

<script setup lang = 'ts'>
import { ref } from 'vue'
const joinRoom = () => {
  // 可以在这里编写加入房间的相关代码
  console.log('加入房间');
};

// 创建房间逻辑
const createRoom = () => {
  // 可以在这里编写创建房间的相关代码
  console.log('创建房间');
};
</script>

<style lang="scss" scoped>
/* 样式 */
.room-management {
  text-align: center;
  padding: 20px;
}

.button-container {
  display: flex;
  justify-content: center;
  gap: 20px;
}

.join-room-button,
.create-room-button {
  padding: 12px 24px;
  font-size: 16px;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.join-room-button {
  background-color: #3498db;
  color: white;
}

.create-room-button {
  background-color: #2ecc71;
  color: white;
}

.join-room-button:hover,
.create-room-button:hover {
  opacity: 0.8;
}
</style>
