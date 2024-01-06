<template>
  <el-container class="full-height">
    <el-header>
      <!-- 可能需要一个logo -->
      <div class="logo"></div>
    </el-header>
    <el-main class="main-content">
      <el-row :gutter="20" class="button-row">
        <el-col :span="12">
          <el-button type="primary" block @click="createRoom">Create Room</el-button>
        </el-col>
        <el-col :span="12">
          <el-button type="success" block @click="joinRoom">Join Room</el-button>
        </el-col>
      </el-row>
    </el-main>
    <el-footer>
      <!-- 可能需要一个FOOTER -->
      <div class="footer-content"></div>
    </el-footer>
  </el-container>
  <el-dialog v-model="createRoomVisible" title="Create Room">
    <el-form :model="createRoomform">
      <el-form-item label="Room Name" :label-width="formLabelWidth">
        <el-input v-model="createRoomform.roomName" autocomplete="off" />
      </el-form-item>
      <!-- <el-form-item label="Public Key" :label-width="formLabelWidth">
        <el-input v-model="createRoomform.publicKey" autocomplete="off" />
      </el-form-item> -->
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="createRoomVisible = false">Cancel</el-button>
        <el-button type="primary" @click="createRoomConfirm">
          Confirm
        </el-button>
      </span>
    </template>
  </el-dialog>
  <el-dialog v-model="joinRoomVisible" title="Join Room">
    <el-form :model="joinRoomVisible">
      <el-form-item label="Room Name" :label-width="formLabelWidth">
        <el-input v-model="joinRoomform.roomName" autocomplete="off" />
      </el-form-item>
      <!-- <el-form-item label="Public Key" :label-width="formLabelWidth">
        <el-input v-model="joinRoomform.publicKey" autocomplete="off" />
      </el-form-item> -->
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="joinRoomVisible = false">Cancel</el-button>
        <el-button type="primary" @click="joinRoomConfirm">
          Confirm
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>
  
<script setup lang="ts">
import { ElContainer, ElHeader, ElFooter, ElMain, ElRow, ElCol, ElButton,ElLoading } from 'element-plus';
import { ref, reactive, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { Room } from '../constant/roomState';
import { initializeWeb3, localRun,createGame,joinGame } from '../services/palletConnet';

onMounted(async () => {

});
const router = useRouter();
const createRoomVisible = ref(false)
const joinRoomVisible = ref(false)
const formLabelWidth = '140px'
const createRoomform = reactive({
  roomName: '',
  publicKey: ''
})

const joinRoomform = reactive({
  roomName: '',
  publicKey: ''
})

const joinRoom = async () => {
  console.log('加入房间');
  joinRoomVisible.value = true
};
const createRoom = async () => {
  createRoomVisible.value = true
};
const createRoomConfirm = async () => {
  console.log('创建房间确认');
  if (localRun) {
    await initializeWeb3();
    createRoomVisible.value = false;
    Room.createGame(createRoomform.roomName);
    router.push({ path: '/pokergame', query: { roomName: createRoomform.roomName, playerName: Room.getRoomState(createRoomform.roomName).players[0].name } });
  }
  else {
    const loading = ElLoading.service({lock: true,text: 'creating game',background: 'rgba(0, 0, 0, 0.7)',})
    const gameId =  await createGame(createRoomform.roomName);
    loading.close();
    router.push({ path: '/pokergame', query: { roomName: createRoomform.roomName, gameId: gameId } });
  }
}

const joinRoomConfirm = async () => {
  console.log('加入房间确认');
  if (localRun) {
    await initializeWeb3();
    joinRoomVisible.value = false
    Room.joinGame(joinRoomform.roomName);
    const roomState = Room.getRoomState(joinRoomform.roomName);
    router.push({ path: '/pokergame', query: { roomName: joinRoomform.roomName, playerName: roomState.players[roomState.players.length - 1].name } });
  }
  else {
    const loading = ElLoading.service({lock: true,text: 'joining game',background: 'rgba(0, 0, 0, 0.7)',})
    const gameId =  await joinGame(joinRoomform.roomName);
    loading.close();
    router.push({ path: '/pokergame', query: { roomName: joinRoomform.roomName, gameId: gameId } });
  }
}
</script>
  
<style scoped>
.full-height {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.logo {
  text-align: center;
}

.main-content {
  flex-grow: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.button-row {
  width: 100%;
  max-width: 600px;
  /* 控制按钮行的最大宽度 */
  text-align: center;
}

.footer-content {
  text-align: center;
}

/* 调整 header 和 footer 的样式 */
.el-header,
.el-footer {
  flex-shrink: 0;
}

/* 调整按钮样式 */
.el-button {
  padding: 10px 20px;
  /* 增加按钮的填充 */
  font-size: 1.2em;
  /* 增加字体大小 */
}
</style>../services/palletConnet