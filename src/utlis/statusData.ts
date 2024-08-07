import { ActionDataAsyncCallback } from './actionCallback';

interface UseFsACtions {
    path: string,
    content?: string
}

export class UseFs {
    path: string
    content!: string;
    constructor(options: UseFsACtions) {
        console.log(options)
        this.path = options.path
        options.content && (this.content = options.content)
    }

    findFile() {
        let params = {
            name: 'find_file',
            item1: this.path,
        }
        return ActionDataAsyncCallback(params.name, params.item1, "");
    }
    readText() {
        let path = this.path;
        return new Promise(async (resolve, reject) => {
            try {
                let params = {
                    name: 'read_text_file',
                    item1: path,
                    item2: ""
                }
                let res = await ActionDataAsyncCallback(params.name, params.item1, "");
                console.log(res);
                resolve(res)
            } catch (error) {
                reject(error)
            }
        })
    }
    writeText(content: string) {
        let path = this.path;
        return new Promise(async (resolve, reject) => {
            try {
                let params = {
                    name: 'write_file',
                    item1: path,
                    item2: content
                }
                let res = await ActionDataAsyncCallback(params.name, params.item1, content);
                console.log(res);
                resolve(res)
            } catch (error) {
                reject(error)

            }
        })
    }

}