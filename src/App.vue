<template>
  <el-container class="full-height">
    <el-header>
      <div class="logo">LOGO</div>
    </el-header>
    <el-main>
      <el-row :gutter="20">
        <el-col :span="12">
          <el-button type="primary" block @click="createRoom">创建房间</el-button>
        </el-col>
        <el-col :span="12">
          <el-button type="success" block @click="joinRoom">加入房间</el-button>
        </el-col>
      </el-row>
    </el-main>
    <el-footer>
      <div class="footer-content">底部信息</div>
    </el-footer>
  </el-container>
  <el-dialog v-model="createRoomVisible" title="创建房间">
    <el-form :model="createRoomform">
      <el-form-item label="房间名" :label-width="formLabelWidth">
        <el-input v-model="createRoomform.roomName" autocomplete="off" />
      </el-form-item>
      <el-form-item label="公钥" :label-width="formLabelWidth">
        <el-input v-model="createRoomform.publicKey" autocomplete="off" />
      </el-form-item>
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="createRoomVisible = false">取消</el-button>
        <el-button type="primary" @click="createRoomVisible = false">
          确定
        </el-button>
      </span>
    </template>
  </el-dialog>
  <el-dialog v-model="joinRoomVisible" title="加入房间">
    <el-form :model="joinRoomVisible">
      <el-form-item label="公钥" :label-width="formLabelWidth">
        <el-input v-model="joinRoomform.publicKey" autocomplete="off" />
      </el-form-item>
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="joinRoomVisible = false">取消</el-button>
        <el-button type="primary" @click="joinRoomVisible = false">
          确定
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ElContainer, ElHeader, ElFooter, ElMain, ElRow, ElCol, ElButton } from 'element-plus';
import { ref, reactive } from 'vue';
const createRoomVisible = ref(false)
const joinRoomVisible = ref(false)
const formLabelWidth = '140px'
const createRoomform = reactive({
  roomName: '',
  publicKey: ''
})
const joinRoomform = reactive({
  publicKey: ''
})

const joinRoom = () => {
  console.log('加入房间');
  joinRoomVisible.value = true
};
const createRoom = () => {
  console.log('创建房间');
  createRoomVisible.value = true
};
</script>

<style scoped>
.full-height {
  height: 100vh;
  /* 全屏高度 */
  display: flex;
  flex-direction: column;
  /* 垂直布局 */
}

.logo {
  text-align: center;
  /* 根据需求添加LOGO的样式 */
}

.row-bg {
  align-items: center;
  /* 垂直居中 */
  min-height: 60vh;
  /* 确保按钮垂直居中 */
}

.footer-content {
  text-align: center;
  /* 根据需求添加底部信息的样式 */
}

/* 适当调整间距和对齐 */
.el-header,
.el-footer {
  flex: 0 0 auto;
}

.el-main {
  flex: 1 0 auto;
  display: flex;
  align-items: center;
  /* 这会垂直居中.el-main的内容 */
  justify-content: center;
  /* 水平居中 */
}</style>