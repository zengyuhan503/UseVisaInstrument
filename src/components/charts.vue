<script setup>
import * as echarts from 'echarts'
import { h, ref, onMounted, watch } from 'vue'
import { FileDoneOutlined, FileAddOutlined } from "@ant-design/icons-vue"
import { UseFileStore } from "../store/useFile";
let useFile = UseFileStore();

let chartCibtauber = ref(null)
let mychart = null;
let volitVals = ref([])
let props = defineProps(["volts", "is_running", "is_volt_case"])
let labels = ref([]);
let toolsIcon = ref({
    saveData: new URL("../assets/images/dowjson.png", import.meta.url),
    updateData: new URL("../assets/images/updatejson", import.meta.url),
    tools: new URL("../assets/images/tools.png", import.meta.url)
})
let updateECharts = (val, label) => {
    let series = []
    let legend = []
    let xAxisData = []
    for (const key in val) {
        let data = val[key];
        if (data.curr.item.length == 0) {
            continue;
        }
        let currlserie = {
            name: key + '电流',
            data: data.curr.item,
            yAxisIndex: 0, // 对应右侧的Y轴
            tooltip: {
                valueFormatter: function (value) {
                    return value + ' mA';
                }
            },
        }
        legend.push(key + '电流')
        series.push(currlserie)

        if (props.is_volt_case) {
            let voltserie = {
                name: key + '功率',
                data: data.volt.item,

                yAxisIndex: 1, // 对应右侧的Y轴
                tooltip: {
                    valueFormatter: function (value) {
                        return value + 'mW';
                    }
                },
            }
            legend.push(key + '功率')
            series.push(voltserie)
        }

    }

    let options = {
        legend: {
            data: legend,
            textStyle: {
                color: "#ffffff"
            }
        },
        series: series
    }
    useFile.setContent(options)
    mychart.setOption(options)
}
let options = {
    title: {
        name: "电量计仪器测试工具"
    },
    toolbox: {
        itemSize: 18,
        itemGap: 25,
    },
    xAxis: {
        type: 'category',
    },
    yAxis: [
        {
            type: 'value',
            name: '电流',
            position: 'left',
            axisLabel: {
                formatter: '{value} mA'
            }
        },
        {
            type: 'value',
            name: '功率',
            position: 'right',
            axisLabel: {
                formatter: '{value} mW'
            }
        }
    ],
    dataZoom: [
        {
            type: 'slider',
            start: 0,
            end: 100
        },
        {
            type: 'inside',
            start: 65,
            end: 85
        },
        {
            type: 'slider',
            yAxisIndex: 0,
            filterMode: 'none'
        },
    ],
    series: [
        {
            name: 'CH1电流',
            data: [],  // 初始数据
            type: 'line',
            yAxisIndex: 0, // 对应左侧的Y轴
            smooth: true,
            tooltip: {
                valueFormatter: function (value) {
                    return value + ' mA';
                }
            },
        }, {
            name: 'CH1功率',
            data: [],  // 初始数据
            yAxisIndex: 1, // 对应右侧的Y轴
            type: 'line',
            tooltip: {
                valueFormatter: function (value) {
                    return value + 'mW';
                }
            },
            smooth: true
        }, {
            name: 'CH2电流',
            data: [],  // 初始数据
            type: 'line',
            yAxisIndex: 0, // 对应左侧的Y轴
            tooltip: {
                valueFormatter: function (value) {
                    return value + ' mA';
                }
            },
            smooth: true
        }, {
            name: 'CH2功率',
            data: [],  // 初始数据
            yAxisIndex: 1, // 对应右侧的Y轴
            type: 'line',
            tooltip: {
                valueFormatter: function (value) {
                    return value + 'mW';
                }
            },
            smooth: true
        }, {
            name: 'CH3电流',
            data: [],  // 初始数据
            yAxisIndex: 0, // 对应左侧的Y轴
            tooltip: {
                valueFormatter: function (value) {
                    return value + ' mA';
                }
            },
            type: 'line',
            smooth: true
        }, {
            name: 'CH3功率',
            data: [],  // 初始数据
            yAxisIndex: 1, // 对应右侧的Y轴
            type: 'line',
            tooltip: {
                valueFormatter: function (value) {
                    return value + 'mW';
                }
            },
            smooth: true
        },
    ],
    tooltip: {
        trigger: 'axis',
        axisPointer: {
            type: 'cross'
        }
    }
};

let clearEcharts = () => {
    mychart.clear();
    mychart.setOption(options)
}
let getChartOptions = () => {
    let options = mychart.getOption();
    return options
}
const updateHistoryData = (content) => {
    mychart.clear();
    mychart.setOption(options)
    mychart.setOption(content)
}
window.addEventListener('resize', () => {
    mychart?.resize()
})
onMounted(() => {
    mychart = echarts.init(chartCibtauber.value);
    mychart.setOption(options);
})
defineExpose({
    updateECharts, clearEcharts, updateHistoryData, getChartOptions
})
</script>
<template>
    <div ref="chartCibtauber" class="chartContainer">
    </div>
</template>
<style lang="less" scoped>
.chartContainer {
    width: 100%;
    height: 100%;
    padding: 35px;
    position: relative;
    background: #100c2a;
    box-sizing: border-box;

    &>div {
        width: 100%;
        height: 100%;
    }
}

.chartContainer .tools {
    position: absolute;
    top: 20px;
    right: 33px;
    width: 45px;
    height: 45px;
    text-align: center;


    img {
        cursor: pointer;
        width: 25px;
    }

    div p {
        color: #fff;
        font-size: 12px;
    }
}
</style>