<template>
  <!-- Form -->
  <el-dialog :v-model="dialogFormVisible"  title="创建房间">
    <el-form :model="form">
      <el-form-item label="房间名" :label-width="formLabelWidth">
        <el-input v-model="form.roomName" autocomplete="off" />
      </el-form-item>
      <el-form-item label="公钥" :label-width="formLabelWidth">
        <el-input v-model="form.publicKey" autocomplete="off" />
      </el-form-item>
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialogFormVisible = false">Cancel</el-button>
        <el-button type="primary" @click="dialogFormVisible = false">
          Confirm
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>
  
<script lang="ts" setup>
import { reactive, ref } from 'vue'
const dialogFormVisible = ref(false)
const formLabelWidth = '140px'
const form = reactive({
  roomName: '',
  publicKey: ''
})

</script>
<style scoped>
.dialog-footer button:first-child {
  margin-right: 10px;
}
</style>
  