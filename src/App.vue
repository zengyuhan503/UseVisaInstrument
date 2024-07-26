<script setup lang="ts">
import { defineComponent, ref, Ref, computed, onMounted, reactive } from 'vue'
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Charts from "./components/charts.vue"
import Head from "./components/head.vue"
import { ListenStart, ActionDataAsyncCallback, ACtionDataCallback, ListenError, FindAllVisainstrs } from "./utlis/actionCallback.ts";
import { message } from 'ant-design-vue';
import moment from 'moment';
import { PlusOutlined } from '@ant-design/icons-vue';
interface FormData {
  [key: string]: {
    case: boolean,
    switch: boolean,
    volt: number,
    curr: number,
    max: {
      [key: string]: number
    }
  }
}

interface VoltData {
  [key: string]: {
    [key: string]: {
      val: number,
      item: number[],
      max: number,
      min: number,
      average: number
    }
  }
}

interface NumericDictionary {
  [key: string]: number[];  // 定义索引签名，允许数字作为键，值为数字
}
interface VisaItem {
  [key: string]: string
}
const VNodes = defineComponent({
  props: {
    vnodes: {
      type: Object,
      required: true,
    },
  },
  render() {
    return this.vnodes;
  },
});

let power_supply = ref(false)
let visa_list = ref<VisaItem[]>([]);

let visa_ip = ref("")
let instr_ip = ref("")
// let instr_ip = ref("172.28.248.113")
let volts: Ref<number | null> = ref(null)
let chartRef: Ref<InstanceType<typeof Charts> | null> = ref(null)
let is_running = ref(false)
let is_stop = ref(false);
let case_select = ref(["1"])
let waitTime = ref(0);
let caseTime = ref(0);
let is_volt_case = computed(() => {
  return (case_select.value.includes('2'))
});

const createVoltEntry = () => {
  const state = reactive({
    "volt": {
      val: 0,
      item: [] as number[],
      max: computed((): number => {
        let vals = state.volt.item;
        return vals.length ? Math.max(...vals) : 0
      }),
      min: computed((): number => {
        let vals = state.volt.item;
        return vals.length ? Math.min(...vals) : 0
      }),
      average: computed((): number => {
        let vals = state.volt.item;
        let sum = vals.reduce((acc, val) => acc + val, 0);
        let average_val = Math.floor((sum / vals.length) * 100) / 100;
        return vals.length ? average_val : 0;
      }),
    },
    "curr": {
      val: 0,
      item: [] as number[],
      max: computed((): number => state.curr.item.length ? Math.max(...state.curr.item) : 0),
      min: computed((): number => {
        let vals = state.curr.item;
        return vals.length ? Math.min(...vals) : 0
      }),
      average: computed((): number => {
        let vals = state.curr.item;
        let sum = vals.reduce((acc, val) => acc + val, 0);
        let average_val = Math.floor((sum / vals.length) * 100) / 100;
        return vals.length ? average_val : 0;
      }),
    }
  })
  return state
};

let volt_data: Ref<VoltData> = ref({
  "CH1": createVoltEntry(),
  "CH2": createVoltEntry(),
  "CH3": createVoltEntry()
});

const time = ref(0);
const intervalId = ref<number | null>(null);

const formattedTime = computed(() => {
  const duration = moment.duration(time.value, 'seconds');
  return moment.utc(duration.asMilliseconds()).format('HH:mm:ss');
});

const startTask = () => {
  let casing = caseTime.value;
  console.log(casing)
  if (casing !== 0) {
    setTimeout(() => {
      actions.value.stop_reading.fun()
    }, casing * 60 * 1000);
  }

}
const startTimer = () => {
  if (intervalId.value !== null) return;
  is_stop.value = false;
  intervalId.value = window.setInterval(() => {
    time.value++;
  }, 1000);
};

const stopTimer = () => {
  if (intervalId.value !== null) {
    clearInterval(intervalId.value);
    intervalId.value = null;
    is_stop.value = true;
  }
};
const resetCaseData = () => {
  volt_data.value.CH1.volt.item = [];
  volt_data.value.CH2.volt.item = [];
  volt_data.value.CH3.volt.item = [];
  volt_data.value.CH1.curr.item = [];
  volt_data.value.CH2.curr.item = [];
  volt_data.value.CH3.curr.item = [];
  chartRef.value.clearEcharts()
}
const resetTimer = async () => {
  is_running.value = false;
  stopTimer();
  await ActionDataAsyncCallback("stop_read", "", "");
  caseTime.value = 0
  waitTime.value = 0;
  time.value = 0;
};
let split_time = ref(500)
let form_state: Ref<FormData> = ref({
  "CH1": {
    case: false,
    volt: 1.5,
    curr: 1.0,
    switch: true,
    max: {
      curr: 10.0,
      volt: 6.0
    }
  },
  "CH2": {
    case: false,
    volt: 1.0,
    curr: 1.0,
    switch: true,
    max: {
      curr: 25.0,
      volt: 2.0
    }
  },
  "CH3": {
    case: false,
    volt: 2.0,
    curr: 1.5,
    switch: true,
    max: {
      curr: 25.0,
      volt: 2.0
    }
  },
})
const handleSwitchChn = (index: string) => {
  let isOpen = form_state.value[index].switch;
  if (!power_supply.value) return false;
  ActionDataAsyncCallback("switch_chn", index, isOpen ? "ON" : "OFF").then(res => {
    console.log(res)
  })
}
function formatSeconds() {
  let time = caseTime.value * 60;
  // 使用 moment 的 duration 方法创建一个时间段
  const duration = moment.duration(time, 'seconds');

  // 格式化输出
  const hours = duration.hours();
  const minutes = duration.minutes();
  const seconds = duration.seconds();

  return ` ${hours} 时 ${minutes} 分 ${seconds} 秒`;
}
const deadline = computed(() => {
  let waiting = waitTime.value * 60 * 1000
  return Date.now() + waiting
});
let openPlaned = ref(false);
let task = true;
let handleCancelTask = () => {
  waitTime.value = 0;
  caseTime.value = 0
  task = false
}
const handleStartTast = () => {
  if (waitTime.value == 0) {
    actions.value.setParams.fun()
  } else {
    task = true;
    openPlaned.value = true;
  }
}
const plannedTest = async () => {
  openPlaned.value = false;
  if (!task) return false;
  actions.value.setParams.fun()
}
let actions = ref({
  powerOn: {
    fun: async function (ip?: string, callfun?: () => void) {
      let ips = ip || visa_ip.value;
      if (ips == '') {
        message.error('请先选择设备IP');
        return false;
      }
      this.loading = true
      let params = {
        name: 'powerOn',
        item1: '',
        item2: ips
      }
      try {
        const res = await ActionDataAsyncCallback(params.name, params.item1, params.item2);
        const data = res as string;
        if (data.includes('ok')) {
          localStorage.setItem('connect_visa_ip', ips)
          message.success(data);
          power_supply.value = true;
          if (callfun) callfun();
        } else {
          message.error(data);
        }
      } catch (error) {
        message.error('请求失败: ' + error);
      }
      this.loading = false
    },
    loading: false
  },
  powerOff: {
    loading: false,
    fun: async function () {
      if (is_running.value == true) {
        message.error('请先结束测试后关闭连接');
        return false;
      }
      this.loading = true
      let params = {
        name: "powerOff",
        item1: "",
        item2: "",
      };
      try {
        const res = await ActionDataAsyncCallback(params.name, params.item1, params.item2);
        const data = res as string;
        if (data.includes('ok')) {
          power_supply.value = false;
          is_running.value = false
          is_stop.value = false;
          chartRef.value?.clearEcharts();
        } else {
          message.error(data);
        }

      } catch (error) {
        message.error(JSON.stringify(error));
      }
      this.loading = false
    }
  },
  setParams: {
    loading: false,
    fun: async function () {

      const filteredKeys = Object.keys(form_state.value).filter(key => form_state.value[key].case);
      // 然后使用 map 方法根据这些键获取完整的对象
      if (filteredKeys.length == 0) {
        message.error('请选择通道');
        return false
      }
      let config = JSON.parse(JSON.stringify(form_state.value))
      for (const key in config) {
        const item = config[key];
        delete item.max
      }
      let params = {
        name: "set_config",
        item1: JSON.stringify(config)
      }
      let res = await ActionDataAsyncCallback(params.name, params.item1, "") as string;
      if (res.indexOf("ok") != -1) {
        setTimeout(() => {
          startTimer()
          actions.value.start_reading.fun();
        }, 1000);
      }
    }
  },
  start_reading: {
    loading: false,
    fun: async function () {
      const filteredKeys = Object.keys(form_state.value).filter(key => form_state.value[key].case);
      // 然后使用 map 方法根据这些键获取完整的对象

      localStorage.setItem('visa_case_data', JSON.stringify(form_state.value));
      let case_data = filteredKeys.map(item => item.slice(2));
      is_running.value = true;
      let params = {
        name: "start_read",
        item1: JSON.stringify(case_data),
        rate: split_time.value
      }
      let label = split_time.value;
      resetCaseData()
      ListenStart(() => {
        startTask()
        startTimer()
      })
      ACtionDataCallback(params.name, params.item1, params.rate + '', function (val) {
        if (is_stop.value) return false;
        console.log(moment().format("YYYY-MM-DD HH:mm:ss"));
        let res = handlePairsVal(val);
        for (const key in res) {
          let data = res[key];
          let volt = parseInt((data[0] * 1000) + '') / 1000;
          let curr = parseInt((data[1] * 1000) + '') / 1000;
          handleSetData(volt, curr, key)
        }
        label += label;
        chartRef.value?.updateECharts(volt_data.value, label)
      })
    },
  },
  stop_reading: {
    fun: async function () {
      resetTimer()

    }
  }
})
let handleSetData = (volt: number, curr: number, ikey: string) => {
  for (const key in volt_data.value) {
    if (key == ikey) {
      volt_data.value[key].volt.val = volt;
      volt_data.value[key].volt.item.push(volt);
      volt_data.value[key].curr.val = curr;
      volt_data.value[key].curr.item.push(curr);
    }
  }
}
let handlePairsVal = (val: string): NumericDictionary => {
  let pairs = val.split(';');
  let result = pairs.reduce((acc, pair): NumericDictionary => {
    const [key, value] = pair.split(':');
    let vals = value.split(',');
    let acc_key = 'CH' + key;
    acc[acc_key] = [parseFloat(vals[0]), parseFloat(vals[1])];
    return acc;
  }, {} as NumericDictionary)
  return result
}
let handleListenErr = async () => {
  await ListenError((err) => {
    message.error(err)
    resetTimer()
    actions.value.powerOff.fun();
  })
}
const addVisaItem = () => {
  if (instr_ip.value == '' || instr_ip == null) {
    message.error('请填写IP地址')
    return false
  }

  actions.value.powerOn.fun(instr_ip.value, () => {
    if (!visa_list.value.filter(item => item.value == instr_ip.value)) {
      visa_list.value.push({
        label: "TCPIP0::" + instr_ip.value,
        value: instr_ip.value
      })
    }
    visa_ip.value = instr_ip.value
  });
}
const handleGetVisaList = async () => {
  try {
    let res = await FindAllVisainstrs() as string[];
    console.log(res)
    for (let i = 0; i < res.length; i++) {
      const text = res[i];
      const regex = /^TCPIP0::([0-9.]+)/;
      const match = text.match(regex);
      if (match) {
        let item = {
          label: match[0],
          value: match[1]
        }
        visa_list.value.push(item)
      }
    }
  } catch (error) {
    message.error(`获取资源失败，${error}`)
  }
}
onMounted(() => {
  handleListenErr()
  handleGetVisaList()
  let connect_visa_ip = localStorage.getItem('connect_visa_ip');
  if (connect_visa_ip) {
    visa_ip.value = connect_visa_ip;
  }
  let case_data = localStorage.getItem('visa_case_data');
  if (case_data) {
    var data = JSON.parse(case_data)
    for (const key in data) {
      if (Object.prototype.hasOwnProperty.call(data, key)) {
        const item = data[key];
        item.case = false
      }
    }
    form_state.value.CH1 = data.CH1;
    form_state.value.CH2 = data.CH2;
    form_state.value.CH3 = data.CH3;
  }
})
</script>

<template>
  <div class="header" data-tauri-drag-region>

    <Head />
  </div>
  <div class="main">
    <div class="container">
      <div class="chartContainer">
        <Charts :volts="volts" ref="chartRef" :is_running="is_running" :is_volt_case="is_volt_case" />
      </div>
      <div class="settings">
        <a-divider orientation="left" style="color: #fff;">连接配置<span style="font-size: 11px;">
            (与设备连接上后才能进行测试)</span></a-divider>

        <div class="forms">
          <a-form layout="inline">
            <a-form-item label="设备" name="cur">
              <a-input-group compact>
                <a-select v-model:value="visa_ip" :options="visa_list" style="width: 210px;">
                  <template #dropdownRender="{ menuNode: menu }">
                    <v-nodes :vnodes="menu" />
                    <a-divider style="margin: 4px 0" />
                    <a-space style="padding: 4px 8px">
                      <a-input ref="inputRef" v-model:value="instr_ip" placeholder="填写IP地址" />
                      <a-button type="text" @click="addVisaItem" :disabled="power_supply"
                        :loading="actions.powerOn.loading">
                        <template #icon>
                          <plus-outlined />
                        </template>
                        添加
                      </a-button>
                    </a-space>
                  </template>
                </a-select>
                <a-button v-if="!power_supply" @click="actions.powerOn.fun()" :loading="actions.powerOn.loading"
                  type="primary">连接设备</a-button>
                <a-button v-else @click="actions.powerOff.fun()" :loading="actions.powerOff.loading" danger
                  type="primary">关闭连接</a-button>
              </a-input-group>

            </a-form-item>
          </a-form>
        </div>
        <a-form :model="form_state" name="basic" autocomplete="off">
          <div v-for="(item, index) in form_state" :key="index">
            <a-divider orientation="left" style="color: #fff;">
              <div class="chn_action">
                <span style="color: #fff;font-size: 14px;margin-right: 20px;">{{ index }}号通道</span>
                <div>
                  <a-checkbox v-model:checked="item.case" :disabled="is_running">
                    <span style="color: #fff;font-size: 14px;">测试</span>
                  </a-checkbox>
                  <div>
                    <span style="color: #fff;font-size: 14px;">通道：</span>
                    <a-switch checked-children="开启" @change="handleSwitchChn(index as string)" :disabled="!item.case"
                      un-checked-children="关" v-model:checked="item.switch" />
                  </div>
                </div>
              </div>

            </a-divider>
            <div>
              <a-row>
                <a-col :span="12">
                  <a-form-item label="电压" name="vol">
                    <a-input-number :step="0.01" :max="item.max.volt" min="1.0" :disabled="!item.case"
                      v-model:value="item.volt" />
                  </a-form-item>
                </a-col>
                <a-col :span="12">
                  <a-form-item label="OCP" name="cur">
                    <a-input-number :step="0.01" :max="item.max.cur" min="1.0" :disabled="!item.case"
                      v-model:value="item.curr" />
                  </a-form-item>
                </a-col>
              </a-row>
            </div>
          </div>
        </a-form>
        <a-divider orientation="left" style="color: #fff;">测试</a-divider>
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item label="采样" name="cur">
              <a-select ref="select" v-model:value="split_time" :disabled="is_running">
                <a-select-option :value="10">10毫秒</a-select-option>
                <a-select-option :value="50">50毫秒</a-select-option>
                <a-select-option :value="100">100毫秒</a-select-option>
                <a-select-option :value="200">200毫秒</a-select-option>
                <a-select-option :value="500">500毫秒</a-select-option>
                <a-select-option :value="700">700毫秒</a-select-option>
                <a-select-option :value="1000">1000毫秒</a-select-option>
                <a-select-option :value="1500">1500毫秒</a-select-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item label="类型">
              <a-checkbox-group v-model:value="case_select" :disabled="is_running">
                <a-checkbox value="1" disabled name="type">电流</a-checkbox>
                <a-checkbox value="2" name="type">电压</a-checkbox>
              </a-checkbox-group>
            </a-form-item>
          </a-col>
        </a-row>
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item label="等待时长" name="cur">
              <a-input-number v-model:value="waitTime" :disabled="is_running" min="0" style="width: 100px;">
                <template #addonAfter>
                  分钟
                </template>
              </a-input-number>
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item label="测试时长" 　name="caseTime">
              <a-input-number v-model:value="caseTime" :disabled="is_running" min="0" style="width: 100px;">
                <template #addonAfter>
                  分钟
                </template>
              </a-input-number>
            </a-form-item>
          </a-col>
        </a-row>
        <a-form-item name="cur">
          <div class="case_btns">
            <a-button v-if="!is_running" :disabled="!power_supply" @click="handleStartTast"
              type="primary">开始测试</a-button>
            <a-button v-else disabled="" type="primary">{{ formattedTime }}</a-button>

            <a-button v-if="!is_stop" @click="stopTimer" :disabled="!is_running">暂停测试</a-button>
            <a-button v-else @click="startTimer" :disabled="!is_running">启动测试</a-button>

            <a-button :disabled="!is_running" @click="actions.stop_reading.fun()" danger type="primary">结束测试</a-button>
          </div>
        </a-form-item>
        <a-divider orientation="left" style="color: #fff;">测试数据</a-divider>
        <div class="case_data">
          <div v-for="(item, index) in volt_data" :key="index" v-show="form_state[index].case">
            <p class="title">{{ index }}号通道数据</p>
            <p>最大电流{{ is_volt_case ? '/电压' : '' }} <br> <span>{{ item.curr.max }}mA {{ is_volt_case ? `/
                ${item.volt.max}V` :
              '' }}</span></p>
            <p>最小电流{{ is_volt_case ? '/电压' : '' }} <br><span>{{ item.curr.min }}mA {{ is_volt_case ? `/
                ${item.volt.min}V` :
              '' }}</span></p>
            <p>当前电流{{ is_volt_case ? '/电压' : '' }} <br><span>{{ item.curr.val || 0 }}mA {{ is_volt_case ? `/
                ${item.volt.val}V` : '' }}</span></p>
            <p>平均电流{{ is_volt_case ? '/电压' : '' }} <br><span>{{ item.curr.average }}mA {{ is_volt_case ? `/
                ${item.volt.average}V` : '' }}</span>
            </p>
            <p>采样次数 <br><span>{{ item.curr.item.length }}次</span></p>
          </div>

        </div>
      </div>

    </div>
    <a-modal :maskClosable="false" :destroyOnClose="true" :closable="false" v-model:open="openPlaned" title="测试任务">
      <template #footer>
        <a-button key="back" danger @click="handleCancelTask">取消测试</a-button>
      </template>
      <div>
        <a-row :gutter="16">
          <a-col :span="24" style="margin-top: 32px">
            <a-statistic-countdown title="你的测试将在倒计时结束后进行" @finish="plannedTest" :value="deadline"
              format=" H 时 m 分 s 秒" />
          </a-col>
          <a-col :span="24" style="margin-top: 32px">
            <a-statistic title="测试时长" :value="formatSeconds()" />
          </a-col>
        </a-row>
      </div>
    </a-modal>
  </div>
</template>

<style>
body,
html {
  width: 100%;
  height: 100%;
  background: transparent;
}

#app {
  width: 100%;
  height: 100%;
  overflow: hidden;
  border-radius: 20px;
  display: flex;
  background: rgb(38, 43, 52);

  flex-direction: column;
}

.main {

  background: rgba(0, 0, 0, 0.2);
  height: calc(100% - 40px);


  .ant-divider::after {
    background: #fff;
  }

  .ant-divider::before {
    background: #fff;
  }

  .ant-form-item label {
    color: #fff !important;
  }

  button:disabled,
  input:disabled {
    background: #d9d9d9 !important;
    color: #000 !important;
  }

  .ant-checkbox-disabled .ant-checkbox-inner:after {
    border-color: #1677ff !important;
  }

  .ant-checkbox-checked:after {
    border: 2px solid #1677ff !important;

    /* background-color: rgba(255, 255, 255, 0.8) !important; */
  }

  .ant-form-item {
    margin-bottom: 10px;
  }

  .ant-checkbox+span {
    color: #fff !important;
  }

  .ant-select-selector {
    background-color: #fff !important;
  }
}

.forms {
  display: flex;
  justify-content: flex-end;
}

.container {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  height: 100%;
}

.settings {
  width: 400px;
  background: rgb(38 43 52);
  padding: 0 10px;


  .btns {
    display: flex;
    margin-bottom: 20px;

    button {
      margin-right: 10px;
      width: 50%;
    }
  }


  .case_btns {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 10px;
  }

  .case_data {
    display: flex;

    &>div {
      width: 50%;
    }

    p.title {
      font-size: 14px;
      font-weight: 600;
    }

    p {
      margin-bottom: 8px;
      color: #fff;
      font-size: 11px;
      text-align: center;
      line-height: 15px;

      span {
        color: #69b1ff;
        font-size: 12px;
        position: relative;
        top: 5px;
      }
    }
  }
}

.chartContainer {
  width: calc(100% - 330px);
  height: 100%;
  position: relative;
}

.chn_action {
  width: 300px;
  display: flex;
  align-items: center;

  &>span {
    margin-right: 12px;
  }

  &>div {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 230px;
  }
}

.ant-select-dropdown {
  width: 270px !important;
}

.ant-input-number-group-addon {
  color: #fff !important;
}
</style>
