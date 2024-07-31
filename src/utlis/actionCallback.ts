import { UnlistenFn, emit, listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';

interface ActionParams {
    name: string,
    item1: any,
    item2: string
}

// 这个TypeScript代码片段定义了一些函数，用于与Tauri应用程序中的事件和API调用进行交互。

// ActionDataAsyncCallback函数是一个异步回调函数，用于执行某个操作并返回结果。它接受三个参数：name、item和item2。函数返回一个Promise对象，用于处理操作的成功或失败。内部逻辑是：

// 监听名为action_res的事件，当事件触发时，检查返回的数据中是否包含error，如果有则调用reject并返回错误信息；
// 如果没有错误，调用resolve并返回数据；
// 在监听之前，通过emit函数发送一个名为action_req的事件，并附带操作的参数。
// ACtionDataCallback函数用于执行某个操作并在操作完成后调用回调函数。它接受四个参数：name、item、item2和callback。函数内部逻辑是：

// 如果已经存在一个监听器unlisten，则先停止之前的监听；
// 监听名为action_data的事件，当事件触发时，调用传入的回调函数并传入事件的负载数据；
// 通过emit函数发送一个名为action_req的事件，并附带操作的参数。

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