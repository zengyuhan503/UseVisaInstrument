import { createApp } from "vue";
import App from "./App.vue";

import { createPinia } from 'pinia';

import Antd from 'ant-design-vue';
import 'ant-design-vue/dist/reset.css'
import "./assets/styles/reset.less"


createApp(App).use(createPinia()).use(Antd).mount("#app");
