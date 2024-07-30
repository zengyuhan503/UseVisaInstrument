<script setup lang="ts">
import { ref, h, onMounted, createVNode } from "vue"
import { ActionDataAsyncCallback } from "../utlis/actionCallback";
import { DeleteOutlined, ExclamationCircleOutlined } from '@ant-design/icons-vue';
import { Modal } from "ant-design-vue";
let emit = defineEmits(["showHistoryItem"])
let historys = ref()
let showItem = ref(0)
let showIndex: number = 0
const historyChange = (e: { target: { value: any; }; }) => {
    let index = e.target.value
    showIndex = index
    changeEchartData(index)
}
const changeEchartData = (index: number) => {
    let item = historys.value[index];
    let content = item.content;
    showIndex = index
    emit('showHistoryItem', content)
}
let uploadHistoryDataLoading = ref(true)
let uploadHistoryData = () => {
    ActionDataAsyncCallback("read_history_data", '', '').then((res: any) => {
        let data = JSON.parse(res);
        if (typeof data == 'object') {
            historys.value = data;
        }
        uploadHistoryDataLoading.value = false;
    })
}
let deleteHistory = (index: number) => {
    Modal.confirm({
        title: '你确定要删除该条数据吗？',
        icon: createVNode(ExclamationCircleOutlined),
        content: createVNode('div', { style: 'color:red;' }, '数据删除后将无法恢复！'),
        okText: "确定",
        cancelText: "取消",
        onOk() {
            ActionDataAsyncCallback("delete_history_data", index + '', '').then(res => {
                historys.value.splice(index, 1)
                if (showIndex == index) {
                    showIndex++;
                    if (showIndex >= historys.value.length) {
                        showIndex = historys.value.length - 1;
                    }
                    changeEchartData(showIndex);
                }
                if(showItem.value>=index){
                    showItem.value--;
                }
            })
        },
    })
}
onMounted(() => {
    uploadHistoryDataLoading.value = true;
    uploadHistoryData()
})
</script>
<template>
    <div class="lists">
        <a-skeleton active :loading="uploadHistoryDataLoading">
            <a-radio-group v-model:value="showItem" @change="historyChange">
                <a-space direction="vertical" :size="20">
                    <div v-for="(item, index) in historys" :key="index" class="list">
                        <a-row>
                            <a-col :span="23">
                                <a-radio :value="index">{{ item.name }}</a-radio>
                            </a-col>
                            <a-col :span="1">
                                <a-button :icon="h(DeleteOutlined)" type="primary" danger size="small"
                                    @click="deleteHistory(index)" />
                            </a-col>
                        </a-row>
                    </div>
                </a-space>
            </a-radio-group>
        </a-skeleton>

    </div>
</template>
<style scoped lang="less">
.lists {
    width: 100%;
    height: calc(100% - 100px);
    overflow-y: auto;

    &::-webkit-scrollbar {
        display: none;
        /* 隐藏滚动条 */
    }

    .item {
        color: #fff;
    }

    .list {
        margin: 0;
        padding: 0;
    }
}
</style>
<style>
.lists .ant-radio-wrapper span {
    color: #fff;
}
</style>