import { defineStore } from 'pinia';

interface SettingData {
    isAutoSave: boolean,
    path: string
}


export const useSettingStore = defineStore('counter', {
    state: () => {
        return {
            isAutoSave: false,
            path: ""
        };
    },
    getters: {
        getState(state) {
            try {
                let settings = localStorage.getItem('visa_setting');
                if (settings) {
                    let data = JSON.parse(settings) as SettingData;
                    state.isAutoSave = data.isAutoSave;
                    state.path = data.path;
                }
            } catch (error) {

            }
            return {
                isAutoSave: state.isAutoSave,
                path: state.path
            }
        }
    },
    actions: {
        setState(data: SettingData) {
            localStorage.setItem('visa_setting', JSON.stringify(data))
            this.isAutoSave = data.isAutoSave;
            this.path = data.path;
        }
    }
});
