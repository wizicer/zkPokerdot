import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { ElLoading } from 'element-plus'

export const localRun: boolean = false;
const wsProviderUrl = 'ws://127.0.0.1:9944';

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


export async function createGame(roomName: string) {
    const { SENDER, injector } = await setupWeb3();
    const api = await setupApi();
    //调用createGame方法
    const tx = api.tx.zkPoker.createGame(roomName);
    //这里会弹出弹窗
    const result = await tx.signAndSend(SENDER, { signer: injector.signer }, ({ status }) => {
        if (status.isInBlock) {
            console.log(`Transaction included at blockHash ${status.asInBlock}`, result);
            api.query.zkPoker.game(roomName)
                .then(gameId => {
                    console.log('Game ID:', gameId);
                    console.log('Game ID human:', gameId.toHuman());
                    result();//取消订阅
                })
                .catch(console.error);
        }
    });
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
