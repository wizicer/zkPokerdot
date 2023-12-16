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
        <el-button type="primary" @click="createRoomConfirm">
          确定
        </el-button>
      </span>
    </template>
  </el-dialog>
  <el-dialog v-model="joinRoomVisible" title="加入房间">
    <el-form :model="joinRoomVisible">
      <el-form-item label="房间名" :label-width="formLabelWidth">
        <el-input v-model="joinRoomform.roomName" autocomplete="off" />
      </el-form-item>
      <el-form-item label="公钥" :label-width="formLabelWidth">
        <el-input v-model="joinRoomform.publicKey" autocomplete="off" />
      </el-form-item>
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="joinRoomVisible = false">取消</el-button>
        <el-button type="primary" @click="joinRoomConfirm">
          确定
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>
  
<script setup lang="ts">
import { ElContainer, ElHeader, ElFooter, ElMain, ElRow, ElCol, ElButton } from 'element-plus';
import { ref, reactive, onMounted } from 'vue';
import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { useRouter } from 'vue-router';

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
let api:ApiPromise;//api调用
let SENDER:string;

const joinRoomform = reactive({
  roomName: '',
  publicKey: ''
})

const joinRoom = () => {
  console.log('加入房间');
  joinRoomVisible.value = true
};
const createRoom = async() => {
  console.log('创建房间');
  createRoomVisible.value = true
};
const createRoomConfirm = async() => {
  console.log('创建房间确认');
  createRoomVisible.value = false
   // returns an array of all the injected sources
  // (this needs to be called first, before other requests)
  const allInjected = await web3Enable('my cool dapp')

  // returns an array of { address, meta: { name, source } }
  // meta.source contains the name of the extension that provides this account
  const allAccounts = await web3Accounts()
  console.log('allAccounts and allInjected',allAccounts, allInjected)

  // the address we use to use for signing, as injected
  SENDER = allAccounts[0].address

  // finds an injector for an address
  const injector = await web3FromAddress(SENDER)
  console.log('SENDER',SENDER)
  //这里会弹出弹窗
  const wsProvider = new WsProvider('ws://192.168.32.223:9944')

  // sign and send our transaction - notice here that the address of the account
  // (as retrieved injected) is passed through as the param to the `signAndSend`,
  // the API then calls the extension to present to the user and get it signed.
  // Once complete, the api sends the tx + signature via the normal process
  // const api = await ApiRx.create({ provider: injector.provider }).toPromise()
  api = await ApiPromise.create({ provider: wsProvider })
  console.log("api", api)
  if (!api) {
    throw new Error('Unable to create ApiRx instance')
  }
  // const result = await api.tx.balances
  //   .transfer('5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y', 123456)
  //   .signAndSend(SENDER, { signer: injector.signer }, (status) => {
  //     console.log('tx status', status)
  //   })
  
  const tx = api.tx.zkPoker.createGame(createRoomform.roomName);

  const result = await tx.signAndSend(SENDER, { signer: injector.signer });

  console.log('result', result)
  const ret = await api.query.zkPoker.game(99);
  console.log('ret', ret)
  router.push({ path: '/pokergame', query: { roomName: createRoomform.roomName } });
}

const joinRoomConfirm = async() => {
  console.log('加入房间确认');
  joinRoomVisible.value = false
    // returns an array of all the injected sources
  // (this needs to be called first, before other requests)
  const allInjected = await web3Enable('my cool dapp')

  // returns an array of { address, meta: { name, source } }
  // meta.source contains the name of the extension that provides this account
  const allAccounts = await web3Accounts()
  console.log('allAccounts and allInjected',allAccounts, allInjected)

  // the address we use to use for signing, as injected
  SENDER = allAccounts[0].address

  // finds an injector for an address
  const injector = await web3FromAddress(SENDER)
  console.log('SENDER',SENDER)
  //这里会弹出弹窗
  const wsProvider = new WsProvider('ws://192.168.32.223:9944')

  // sign and send our transaction - notice here that the address of the account
  // (as retrieved injected) is passed through as the param to the `signAndSend`,
  // the API then calls the extension to present to the user and get it signed.
  // Once complete, the api sends the tx + signature via the normal process
  // const api = await ApiRx.create({ provider: injector.provider }).toPromise()
  api = await ApiPromise.create({ provider: wsProvider })
  console.log("api", api)
  if (!api) {
    throw new Error('Unable to create ApiRx instance')
  }
  // const result = await api.tx.balances
  //   .transfer('5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y', 123456)
  //   .signAndSend(SENDER, { signer: injector.signer }, (status) => {
  //     console.log('tx status', status)
  //   })
  
  //const tx = api.tx.zkPoker.createGame(567);

  //const result = await tx.signAndSend(SENDER, { signer: injector.signer });

  //console.log('result', result)
 
  const tx = await api.tx.zkPoker.joinGame(joinRoomform.roomName);
  const result = await tx.signAndSend(SENDER, { signer: injector.signer });
  console.log('result', result)
  router.push({ path: '/pokergame', query: { roomName: joinRoomform.roomName } });
}
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
}
</style>