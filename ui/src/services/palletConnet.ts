import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { ElLoading } from 'element-plus'

export const localRun: boolean = false;
const wsProviderUrl = 'ws://127.0.0.1:9944';
//拆分初始化函数
async function setupWeb3() {
    const allInjected = await web3Enable('my cool dapp');
    if (!allInjected.length) {
        throw new Error('No injected sources available');
    }

    const allAccounts = await web3Accounts();
    if (!allAccounts.length) {
        throw new Error('No accounts available');
    }

    const SENDER = allAccounts[0].address;
    const injector = await web3FromAddress(SENDER);
    return { SENDER, injector };
}
//拆分初始化函数
async function setupApi() {
    const wsProvider = new WsProvider(wsProviderUrl);
    const api = await ApiPromise.create({ provider: wsProvider });
    if (!api) {
        throw new Error('Unable to create ApiRx instance');
    }
    return api;
}

export async function initializeWeb3() {
    const { SENDER, injector } = await setupWeb3();
    const api = await setupApi();
    const tx = api.tx.templateModule.doSomething(99);
    const result = await tx.signAndSend(SENDER, { signer: injector.signer });
    console.log('result', result);
    return result;
}
export const delay = async (ms: number) => {
    return new Promise(resolve => { setTimeout(resolve, ms) });
}

export const loading = async (ms: number, text: string) => {
    const loading = ElLoading.service({
        lock: true,
        text: text,
        background: 'rgba(0, 0, 0, 0.7)',
    })
    await delay(ms);
    loading.close();
}

//创建游戏
export async function createGame(roomName: string): Promise<number> {
    const { SENDER, injector } = await setupWeb3();
    const api = await setupApi();
    const tx = api.tx.zkPoker.createGame(roomName);
    let ret = -1;

    return new Promise<number>((resolve, reject) => {
        tx.signAndSend(SENDER, { signer: injector.signer }, async ({ status }) => {
            if (status.isInBlock) {
                console.log(`Transaction included at blockHash ${status.asInBlock}`);
                try {
                    const gameId = await api.query.zkPoker.game(roomName);
                    console.log('Gameid:', gameId);
                    console.log('Gameid human:', gameId.toHuman());
                    console.log('Gameid toJSON:', gameId.toJSON());
                    ret = gameId.toJSON() as number;
                    resolve(ret);
                } catch (error) {
                    reject(error);
                }
            }
        });
    });
}
//加入游戏
export async function joinGame(roomName: string): Promise<number> {
    const { SENDER, injector } = await setupWeb3();
    const api = await setupApi();
    const tx = api.tx.zkPoker.joinGame(roomName);
    let ret = -1;

    return new Promise<number>((resolve, reject) => {
        tx.signAndSend(SENDER, { signer: injector.signer }, async ({ status }) => {
            if (status.isInBlock) {
                console.log(`Transaction included at blockHash ${status.asInBlock}`);
                try {
                    const gameId = await api.query.zkPoker.game(roomName);
                    console.log('Gameid:', gameId);
                    console.log('Gameid human:', gameId.toHuman());
                    console.log('Gameid toJSON:', gameId.toJSON());
                    ret = gameId.toJSON() as number;
                    resolve(ret);
                } catch (error) {
                    reject(error);
                }
            }
        });
    });
}
//查询玩家
export const queryPlayer = async (gameId: string): Promise<T> => {
    const api = await setupApi();
    return await api.query.zkPoker.gamePlayers(gameId);
}
export async function initPokerGame() {
    const api = await setupApi();
    // 订阅系统事件
    api.query.system.events((events: any) => {
        events.forEach((record: any) => {
            // 获取事件数据
            const { event, phase } = record;
            const types = event.typeDef;

            // 检查是否是特定的 pallet 和方法
            if (event.section === 'zkPoker') {
                // 打印事件的详细信息
                console.log(`Event: ${event.section}.${event.method}`);

                event.data.forEach((data: any, index: any) => {
                    console.log(`\t${types[index].type}: ${data.toString()}`);
                });
            }
        });
    });
}
