<script setup>
import * as echarts from 'echarts'
import { h, ref, onMounted, watch } from 'vue'
import { FileDoneOutlined, FileAddOutlined } from "@ant-design/icons-vue"
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
            name: key + '号电流',
            data: data.curr.item,
            yAxisIndex: 0, // 对应右侧的Y轴
            tooltip: {
                valueFormatter: function (value) {
                    return value + ' mA';
                }
            },
        }
        legend.push(key + '号电流')
        series.push(currlserie)

        if (props.is_volt_case) {
            let voltserie = {
                name: key + '号电压',
                data: data.volt.item,

                yAxisIndex: 1, // 对应右侧的Y轴
                tooltip: {
                    valueFormatter: function (value) {
                        return value + 'V';
                    }
                },
            }
            legend.push(key + '号电压')
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
    mychart.setOption(options)
}
let options = {
    title: {
        name: "电量计仪器测试工具"
    },
    toolbox: {
        itemSize: 28,
        feature: {
            // saveAsImage: {
            //     title: "保存图片"
            // },
            // myTool2: {
            //     show: true,
            //     title: '工具组',
            //     icon: 'image://' + toolsIcon.value.tools.href,
            //     onclick: function () {
            //         alert('myToolHandler2')
            //     }
            // }
        }
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
            name: '电压',
            position: 'right',
            axisLabel: {
                formatter: '{value} V'
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
            name: 'o1号电流',
            data: [],  // 初始数据
            type: 'line',
            yAxisIndex: 0, // 对应左侧的Y轴
            smooth: true
        }, {
            name: 'o1号电压',
            data: [],  // 初始数据
            yAxisIndex: 1, // 对应右侧的Y轴
            type: 'line',
            smooth: true
        }, {
            name: 'o2号电流',
            data: [],  // 初始数据
            type: 'line',
            yAxisIndex: 0, // 对应左侧的Y轴
            smooth: true
        }, {
            name: 'o2号电压',
            data: [],  // 初始数据
            yAxisIndex: 1, // 对应右侧的Y轴
            type: 'line',
            smooth: true
        }, {
            name: 'o3号电流',
            data: [],  // 初始数据
            yAxisIndex: 0, // 对应左侧的Y轴
            type: 'line',
            smooth: true
        }, {
            name: 'o3号电压',
            data: [],  // 初始数据
            yAxisIndex: 1, // 对应右侧的Y轴
            type: 'line',
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
window.addEventListener('resize', () => {
    mychart?.resize()
})
onMounted(() => {
    mychart = echarts.init(chartCibtauber.value);
    mychart.setOption(options);
})
defineExpose({
    updateECharts, clearEcharts
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