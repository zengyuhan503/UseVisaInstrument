<script setup lang="ts">
import { ref, h, onMounted, createVNode, watch } from "vue"
import { ActionDataAsyncCallback } from "../utlis/actionCallback";
import { DeleteOutlined, ExclamationCircleOutlined } from '@ant-design/icons-vue';
import { message, Modal } from "ant-design-vue";

interface HistoryItem {
    name: string,
    content: any,
}
interface ManagerOptions {
    label: string,
    value: number
}
let emit = defineEmits(["showHistoryItem"])
let historys = ref<HistoryItem[]>([])
let showItem = ref(null)
let showIndex: number = 0
let isManager = ref(true)
let checkAlls = ref(false);
let indeterminates = ref(false);
let plainOptions = ref<ManagerOptions[]>([])
let historyGroup = ref<number[]>([]);
const changeListManager = (type: any) => {
    isManager.value = type
    let options = [];
    if (!isManager.value) {
        showItem.value = null;
    } else {
        historyGroup.value = []
    }
    options = historys.value.map((item, key) => {
        let option: ManagerOptions = {
            label: item.name,
            value: key
        }
        return option
    })
    plainOptions.value = options
}
const onCheckAllChange = () => {
    if (checkAlls.value) {
        let keys = historys.value.map((item, key) => {
            return key
        })
        historyGroup.value = keys
    } else {
        historyGroup.value = []
    }
}
watch(() => historys.value, lists => {
    let options = [];
    options = lists.map((item, key) => {
        let option: ManagerOptions = {
            label: item.name,
            value: key
        }
        return option
    })
    plainOptions.value = options
})
watch(() => historyGroup.value, val => {
    indeterminates.value = !!val.length && val.length < plainOptions.value.length
    checkAlls.value = val.length === plainOptions.value.length;
})
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
        historys.value = []
        if (typeof data == 'object') {
            historys.value = data;
            console.log(historys.value)
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
            ActionDataAsyncCallback("delete_history_data", index + '', '').then(() => {
                historys.value.splice(index, 1)
                if (showIndex == index) {
                    showIndex++;
                    if (showIndex >= historys.value.length) {
                        showIndex = historys.value.length - 1;
                    }
                    changeEchartData(showIndex);
                }
                if (showItem.value && showItem.value >= index) {
                    showItem.value--;
                }
            })
        },
    })
}
const removeCheckHistory = () => {
    if (historyGroup.value.length == 0) {
        message.error('请选择你要删除的数据');
        return false
    };
    Modal.confirm({
        title: '你确定要删除这些数据吗？',
        icon: createVNode(ExclamationCircleOutlined),
        content: createVNode('div', { style: 'color:red;' }, '数据删除后将无法恢复！'),
        okText: "确定",
        cancelText: "取消",
        onOk() {
            historyGroup.value.sort((a, b) => b - a);
            let reqs = historyGroup.value.map(index => {
                return ActionDataAsyncCallback("delete_history_data", index + '', '')
            })
            Promise.all(reqs).then((res) => {
                historyGroup.value = [];
                uploadHistoryData()
            })
        },
    })
}
defineExpose({ changeListManager })
onMounted(() => {
    uploadHistoryDataLoading.value = true;
    uploadHistoryData()
})
</script>
<template>
    <div class="lists" v-if="isManager">
        <a-skeleton active :loading="uploadHistoryDataLoading">
            <a-radio-group v-model:value="showItem" @change="historyChange">
                <a-space direction="vertical" :size="10">
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
    <div class="checks" v-else>
        <div class="buts">
            <a-checkbox v-model:checked="checkAlls" :indeterminate="indeterminates" @change="onCheckAllChange">
                选择所有
            </a-checkbox>
            <div>
                <a-button type="primary" size="small" @click="removeCheckHistory" danger>删除选择</a-button>
            </div>
        </div>
        <div class="items">
            <a-checkbox-group v-model:value="historyGroup" name="checkboxgroup" :options="plainOptions" />
        </div>
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

.checks {
    height: 100%;

    .buts {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .items {
        height: calc(100% - 110px);
        margin-top: 10px;
        overflow: auto;

        &::-webkit-scrollbar {
            display: none;
            /* 隐藏滚动条 */
        }
    }
}
</style>
<style>
.lists .ant-radio-wrapper span {
    color: #fff;
}
</style>