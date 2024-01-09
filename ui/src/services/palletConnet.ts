import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { ElLoading } from 'element-plus'
import { pokerNumber, shuffleDeckNumber,createPokerCard } from '../constant/poker';

export const localRun: boolean = false;

class SubstrateConnection {
    private static instance: SubstrateConnection;
    private api?: ApiPromise;

    private constructor() { }

    public static getInstance(): SubstrateConnection {
        if (!this.instance) {
            this.instance = new SubstrateConnection();
        }
        return this.instance;
    }

    public async setupApi(): Promise<ApiPromise> {
        if (!this.api) {
            const wsProvider = new WsProvider('ws://127.0.0.1:9944');
            this.api = await ApiPromise.create({ provider: wsProvider });
        }
        return this.api;
    }

    public async setupWeb3() {
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
}

export default SubstrateConnection;

export async function initializeWeb3() {
    const substrateConnection = SubstrateConnection.getInstance();
    const { SENDER, injector } = await substrateConnection.setupWeb3();
    const api = await substrateConnection.setupApi();
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
    const substrateConnection = SubstrateConnection.getInstance();
    const { SENDER, injector } = await substrateConnection.setupWeb3();
    const api = await substrateConnection.setupApi();
    const allAccounts = await web3Accounts();
    const tx = api.tx.zkPoker.createGame(roomName, allAccounts[0].meta.name as string);
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
    const substrateConnection = SubstrateConnection.getInstance();
    const { SENDER, injector } = await substrateConnection.setupWeb3();
    const api = await substrateConnection.setupApi();
    const allAccounts = await web3Accounts();
    const tx = api.tx.zkPoker.joinGame(roomName, allAccounts[0].meta.name as string);
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
//准备游戏
export async function readyGame(gameId: string): Promise<any> {
    const substrateConnection = SubstrateConnection.getInstance();
    const { SENDER, injector } = await substrateConnection.setupWeb3();
    const api = await substrateConnection.setupApi();
    const Decks = await api.query.zkPoker.gameDecks(gameId);
    let pokers = Decks.toHuman();
    console.log(pokers);
    if (Decks.toHuman().length === 0) {
        pokers = shuffleDeckNumber(pokerNumber);
    }
    else {
        pokers = shuffleDeckNumber(Decks.toHuman());
    }
    console.log(pokers);
    const tx = await api.tx.zkPoker.shuffle(gameId, pokers);
    let ret = -1;

    return new Promise<number>((resolve, reject) => {
        tx.signAndSend(SENDER, { signer: injector.signer }, async ({ status }) => {
            if (status.isInBlock) {
                console.log(`Transaction included at blockHash ${status.asInBlock}`);
                try {
                    const retPokers = await api.query.zkPoker.playerCards(SENDER);
                    console.log('retPokers:', retPokers);
                    console.log('retPokers toJSON:', retPokers.toJSON());
                    ret = 1;
                    resolve(ret);
                } catch (error) {
                    reject(error);
                }
            }
        });
    });
}
//抢地主
export const robLandlord = async (gameId: string): Promise<number> => {
    const substrateConnection = SubstrateConnection.getInstance();
    const { SENDER, injector } = await substrateConnection.setupWeb3();
    const api = await substrateConnection.setupApi();
    const tx = api.tx.zkPoker.call(gameId);
    let ret = -1;

    return new Promise<number>((resolve, reject) => {
        tx.signAndSend(SENDER, { signer: injector.signer }, async ({ status }) => {
            if (status.isInBlock) {
                console.log(`Transaction included at blockHash ${status.asInBlock}`);
                try {
                    const gameLeader = await api.query.zkPoker.gameLeader(gameId);
                    console.log('gameLeader toJSON:', gameLeader.toJSON());
                    ret = gameLeader.toJSON() as number;
                    resolve(ret);
                } catch (error) {
                    reject(error);
                }
            }
        });
    });
 }
//查询玩家
export const queryPlayer = async (gameId: string): Promise<any> => {
    const substrateConnection = SubstrateConnection.getInstance();
    const api = await substrateConnection.setupApi();
    return await api.query.zkPoker.gamePlayers(gameId);
}
//查询本玩家牌
export const queryCards = async (): Promise<any> => {
    const substrateConnection = SubstrateConnection.getInstance();
    const { SENDER, injector } = await substrateConnection.setupWeb3();
    const api = await substrateConnection.setupApi();
    return await api.query.zkPoker.playerCards(SENDER);
}
//查询底牌
export const queryBottomCards = async (gameId:any): Promise<any> => {
    const substrateConnection = SubstrateConnection.getInstance();
    const { SENDER, injector } = await substrateConnection.setupWeb3();
    const api = await substrateConnection.setupApi();
    return await api.query.zkPoker.bottomCards(gameId);
}
//设置玩家卡牌
async function setCards(playerMiddle: any,) {
    const cards = await queryCards();
    console.log('cards:', cards.toJSON());
    const myCard = createPokerCard(cards.toJSON()).sort((a, b) => b.id - a.id);
    playerMiddle.value.hands = myCard;
}
//设置底牌
async function setBottomCards(topThree: any,gameId:any) {
    const cards = await queryBottomCards(gameId);
    console.log('bottomcards:', cards);
    console.log('bottomcards human:', cards.toHuman());
    const cardsHuman = cards.toHuman(); 
    // 将字符串数组转换为数字数组,我也不知道为什么变成字符串数组了，为什么playerCards没有变成字符串
    const cardsNumberArray = cardsHuman.map(card => parseInt(card));
    const myCard = createPokerCard(cardsNumberArray).sort((a, b) => b.id - a.id);
    topThree.value.hands = myCard;
}
//循环设置名字
async function setName(gameId: string, playerMiddle: any, playerLeft: any, playerRight: any) {
    const players = await queryPlayer(gameId);
    console.log('players human:', players.toHuman());
    const allInjected = await web3Enable('my cool dapp');
    if (!allInjected.length) {
        throw new Error('No injected sources available');
    }
    const allAccounts = await web3Accounts();
    playerMiddle.value.name = allAccounts[0].meta.name as string;
    console.log(allAccounts[0].meta.name);
    let startIndex = -1;
    for (let i = 0; i < players.toHuman().length; i++) {
        if (allAccounts[0].address === players.toHuman()[i]) {
            startIndex = i;
            break;
        }
    }
    if (startIndex !== -1) {
        for (let j = 0; j < players.toHuman().length; j++) {
            if (j === 0) {
                continue;
            }
            const currentIndex = (startIndex + j) % players.toHuman().length;
            if (j === 1) {
                playerRight.value.name = players.toHuman()[currentIndex];
            }
            if (j === 2) {
                playerLeft.value.name = players.toHuman()[currentIndex];
            }
        }
    } else {
        console.log('Player not found');
    }
}

//事件查询的回调函数
async function handleEvent(gameId: string, api: any, playerMiddle: any, playerLeft: any, playerRight: any,topThree :any) {
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
                if (event.method === 'GamerJoined') {
                    setName(gameId, playerMiddle, playerLeft, playerRight);
                }
                if (event.method === 'PlayerAllPrepared') {
                    setCards(playerMiddle);
                }
                if (event.method === 'GameCurPlayerId') {
                    setCards(playerMiddle);
                    setBottomCards(topThree,gameId);
                }
            }
        });
    });
}
export async function initPokerGame(gameId: string, playerMiddle: any, playerLeft: any, playerRight: any,topThree :any) {
    const substrateConnection = SubstrateConnection.getInstance();
    const api = await substrateConnection.setupApi();
    // 订阅系统事件
    await handleEvent(gameId, api, playerMiddle, playerLeft, playerRight,topThree);
    await setName(gameId, playerMiddle, playerLeft, playerRight);
}
