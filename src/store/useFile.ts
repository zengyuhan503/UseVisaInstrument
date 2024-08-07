
import { UseFs } from '../utlis/statusData';
import { defineStore } from 'pinia';
import { ActionDataAsyncCallback } from '../utlis/actionCallback';


export const UseFileStore = defineStore("fileData", {
    state: () => {
        return {
            useFile: new UseFs({ path: "" }),
            content: null,
            isCase: false,
            startTime: '',
            endTime: ""
        }
    },
    getters: {
        getUseFile(state) {
            return state.useFile
        },
        getTimeProps(state) {
            return `${state.startTime}_${state.endTime}`
        }
    },
    actions: {
        setTime(start: string) {
            this.startTime = start;
        },
        setEndTime(end: string) {
            this.endTime = end;
        },
        setContent(contnet: any) {
            this.content = contnet
        },
        stopFile(end: string,caseName:string) {
            this.endTime = end;
            let params = {
                name: `${this.startTime} ~ ${this.endTime}`,
                content: this.content,
                caseName:caseName
            }
            let item1 = JSON.stringify(params)
            ActionDataAsyncCallback("set_history_data", item1, '')
        }
    }
})