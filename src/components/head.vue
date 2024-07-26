<script setup lang="ts">
import { ref, Ref } from 'vue';
import { appWindow } from '@tauri-apps/api/window';
import Setting from './setting.vue';

import { useSettingStore } from '../store/setting';



type icon = {
    "normal": string,
    "hover": string,
    "isHover": boolean,
    action: () => void
}
interface HeadIcon {
    [key: number]: icon
}
let head_ioncs = ref({
    1: {
        "normal": new URL("../assets/images/header-icon (1).png", import.meta.url) as unknown as string,
        "hover": new URL("../assets/images/header-icon (4).png", import.meta.url) as unknown as string,
        isHover: false,
        action: () => { appWindow.minimize() }
    },
    2: {
        "normal": new URL("../assets/images/header-icon (2).png", import.meta.url) as unknown as string,
        "hover": new URL("../assets/images/header-icon (5).png", import.meta.url) as unknown as string,
        isHover: false,
        action: () => { appWindow.toggleMaximize() }
    },
    3: {
        "normal": new URL("../assets/images/header-icon (3).png", import.meta.url),
        "hover": new URL("../assets/images/header-icon (6).png", import.meta.url),
        isHover: false,
        action: () => { appWindow.close() }
    },

} as unknown as HeadIcon)


const getIcon = (item: icon): string => {
    let url = (item.isHover ? item.hover : item.normal) as unknown as URL;
    return url.href;
}

const hoverEffect = (item: icon) => {
    item.isHover = !item.isHover;
}
let settingRef: Ref<InstanceType<typeof Setting> | null> = ref(null)
let useSetting = useSettingStore()
const handleOkSetting = () => {
    let data = settingRef.value?.settingState;
    if (data) { useSetting.setState(data) }
}
</script>
<template>
    <div class="customization-page app-header " data-tauri-drag-region>
        <div class="title-bar">
            <img src="../assets/images/curr.png" class="logo" alt="">
            <span class="title">KS电源程控测试</span>
        </div>
        <div class="action-icons">
            <a-space warp size="middle">
                <!-- <a-popover placement="topLeft">
                    <template #content>
                        <a-space warp size="middle">
                            <a-tooltip title="数据储存">
                                <a-button type="primary" shape="circle" :icon="h(FileDoneOutlined)" />
                            </a-tooltip>
                            <a-tooltip title="查看历史数据">
                                <a-button type="primary" shape="circle" :icon="h(FileAddOutlined)" />
                            </a-tooltip>
                        </a-space>
                    </template>
                    <template #title>
                        <span>工具组件</span>
                    </template>
                    <div>
                        <span>
                            <img src="../assets/images/setting.png" style="width: 24px;" alt="" srcset="">
                        </span>
                    </div>
                </a-popover> -->
                <span v-for="(item, index) in head_ioncs" :key="index">
                    <img :src="getIcon(item)" @click="item.action" @mouseover="hoverEffect(item)"
                        @mouseleave="hoverEffect(item)" alt="">
                </span>
            </a-space>
        </div>
    </div>
    <!-- <a-modal :maskClosable="false" v-model:open="isSetting" okText="确定" cancelText="取消" title="数据管理设置"
        @ok="handleOkSetting">
        <Setting ref="settingRef" />
    </a-modal> -->
</template>
<style lang="less">
.app-header {
    height: 40px;
    // background: #262b34;
    box-shadow: 0 0 6px rgba(0, 0, 0, 0.45);
    overflow: hidden;
    display: flex;
    justify-content: space-between;
    align-items: center;
    user-select: none;
    padding: 0 20px;
    position: relative;
    z-index: 99;

    .title-bar {
        display: flex;
        align-items: center;
        color: #fff;
    }

    .logo {
        width: 22px;
        margin-right: 15px;
    }

    .title {
        font-size: 16px;
    }

    .action-icons {
        display: flex;
        align-items: center;

        span {
            display: inline-block;
            width: 40px;
            height: 40px;
            display: flex;
            align-items: center;
            justify-content: center;

            img {
                width: 30px;
                cursor: pointer;
                // opacity: 0.5;

                &:hover {
                    opacity: 1;
                }
            }
        }
    }
}
</style>
