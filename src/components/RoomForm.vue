<template>
    <div class="form-overlay" v-show="visible">
      <div class="form-container">
        <button class="close-btn" @click="closeForm">X</button>
        <h2>创建房间</h2>
        <form @submit.prevent="submitForm" class="form-content">
          <div class="form-group">
            <label for="roomName">房间名：</label>
            <input type="text" id="roomName" v-model="roomName" required>
          </div>
          <!-- 其他表单输入 -->
  
          <div class="form-buttons">
            <button type="submit">提交</button>
            <button type="button" class="cancel-btn" @click="closeForm">取消</button>
          </div>
        </form>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, defineProps, defineEmits } from 'vue';
  
  const emits = defineEmits(['submit', 'close']);
  const props = defineProps({
    visible: Boolean
  });
  
  const roomName = ref('');
  
  const submitForm = () => {
    emits('submit', roomName.value);
    roomName.value = '';
    closeForm();
  };
  
  const closeForm = () => {
    emits('close');
  };
  </script>
  
  <style scoped>
  /* 样式 */
  .form-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .form-container {
    background: #fff;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.3);
    text-align: center;
    position: relative;
  }
  
  .close-btn {
    position: absolute;
    top: 10px;
    right: 10px;
    border: none;
    background: none;
    font-size: 1.2em;
    cursor: pointer;
    color: #555;
  }
  
  .form-content {
    margin-top: 20px;
  }
  
  .form-group {
    margin-bottom: 15px;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }
  
  label {
    margin-bottom: 5px;
  }
  input {
    padding: 8px;
    border: 1px solid #ccc;
    border-radius: 4px;
    width: calc(100% - 20px);
    box-sizing: border-box;
  }
  button {
    padding: 10px 20px;
    margin: 0 5px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.3s ease;
  }
  button:hover {
    opacity: 0.8;
  }
  .cancel-btn {
    background-color: #e74c3c;
    color: #fff;
  }
  </style>