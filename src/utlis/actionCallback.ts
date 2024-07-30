import { UnlistenFn, emit, listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';

interface ActionParams {
    name: string,
    item1: any,
    item2: string
}
export const ActionDataAsyncCallback = (name: string, item: any, item2: string) => {
    return new Promise(async (resolve, reject) => {
        try {
            let unlisten = await listen("action_res", e => {
                let data = e.payload as string;
                if (data.indexOf('error') != -1) {
                    reject(data)
                    return false;
                }
                unlisten()
                resolve(data)
            })
            let params: ActionParams = {
                name: name,
                item1: item,
                item2: item2
            }
            console.log(params)
            await emit("action_req", params)
        } catch (error) {
            reject(error)
        }
    })
}

let unlisten: UnlistenFn | null = null
export const ACtionDataCallback = async (name: string, item: string, item2: string, callback: (arg0: any) => void) => {
    if (unlisten) unlisten();
    unlisten = await listen("action_data", e => {
        callback(e.payload);
    })
    let params: ActionParams = {
        name: name,
        item1: item,
        item2: item2
    }
    await emit("action_req", params)
}


let unlistenErr: UnlistenFn | null = null;
export const ListenError = async (errfuns: (arg0: any) => void) => {
    if (unlistenErr) unlistenErr();
    unlistenErr = await listen("action_error", e => {
        console.log("action_error", e.payload);
        errfuns(e.payload)
    })

}
let unlistenStart: UnlistenFn | null = null;
export const ListenStart = async (callback: () => void) => {
    if (unlistenStart) unlistenStart();
    unlistenStart = await listen("start_read_case", _e => {
        callback()
    })
}


export const FindAllVisainstrs = async () => {
    return await invoke("find_all_visa_instrument")
}