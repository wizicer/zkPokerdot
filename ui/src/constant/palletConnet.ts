import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { ElLoading } from 'element-plus'

export async function initializeWeb3() {

    const allInjected = await web3Enable('my cool dapp');
    if (!allInjected.length) {
        throw new Error('No injected sources available');
    }

    const allAccounts = await web3Accounts();
    console.log(allAccounts, allInjected);
    if (!allAccounts.length) {
        throw new Error('No accounts available');
    }

    const SENDER = allAccounts[0].address;

    const injector = await web3FromAddress(SENDER);
    console.log(injector);

    const wsProvider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider: wsProvider });
    console.log("api", api);

    if (!api) {
        throw new Error('Unable to create ApiRx instance');
    }
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
export const localRun: boolean = false;

export async function createGame(roomName: string) {
    // 激活与浏览器扩展的连接
    const allInjected = await web3Enable('my cool dapp')

    // 获取所有通过Polkadot扩展注入的账户
    const allAccounts = await web3Accounts()
    console.log('allAccounts and allInjected', allAccounts, allInjected)

    // 选择第一个账户的地址作为发送者
    const SENDER = allAccounts[0].address

    // 为指定的地址找到一个注入器
    const injector = await web3FromAddress(SENDER)
    console.log('SENDER', SENDER)
    // 连接到Polkadot节点
    const wsProvider = new WsProvider('ws://127.0.0.1:9944')
    // 创建一个与Polkadot区块链交互的API实例
    const api = await ApiPromise.create({ provider: wsProvider })
    console.log("api", api)
    if (!api) {
        throw new Error('Unable to create ApiRx instance')
    }

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
    const allInjected = await web3Enable('my cool dapp');
    if (!allInjected.length) {
        throw new Error('No injected sources available');
    }

    const allAccounts = await web3Accounts();
    console.log(allAccounts, allInjected);
    if (!allAccounts.length) {
        throw new Error('No accounts available');
    }

    const SENDER = allAccounts[0].address;

    const injector = await web3FromAddress(SENDER);
    console.log(injector);

    const wsProvider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider: wsProvider });
    console.log("api", api);

    if (!api) {
        throw new Error('Unable to create ApiRx instance');
    }
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
