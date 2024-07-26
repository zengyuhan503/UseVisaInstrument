<script lang="ts" setup>
import { ref, onMounted } from "vue";
import { useSettingStore } from '../store/setting';
import { save } from "@tauri-apps/api/dialog"
import moment from "moment";
let useStting = useSettingStore();

let settingState = ref({
    isAutoSave: false,
    path: ""
})
const savePath = async () => {
    let name = moment().format("YYYY-MM-DD")
    let path = await save({
        title: "保存文件",
        filters: [{
            name: "json",
            extensions: ["json"]
        }],
        defaultPath: `/log/${name}`
    })
    settingState.value.path = path as string;
}
onMounted(() => {
    settingState.value = useStting.getState;
})

defineExpose({
    settingState
})
</script>
<template>
    <div>
        <p>需要进行设置后，数据才会自动保存</p>
        <a-form labelAlign="left" :model="settingState" :label-col="{ style: { width: '110px' } }">
            <a-form-item label="文件保存地址" name="path">
                <a-input-group compact>
                    <a-input disabled placeholder="请先选择保存地址" style="width: calc(100% - 120px)"
                        v-model:value="settingState.path" />
                    <a-button type="primary" @click="savePath">选择保存地址</a-button>
                </a-input-group>
            </a-form-item>
            <a-form-item label="自动存储数据">
                <a-switch v-model:checked="settingState.isAutoSave" />
            </a-form-item>
        </a-form>
    </div>
</template>