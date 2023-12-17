import type { PokerCard } from './poker';

export interface Player {
  name: string;
  hands: PokerCard[];
  isLandlord: boolean;
  isReady: boolean;
}

export interface RoomState {
  players: Player[];
  currentPlayerIndex: number;
  roomState: 'waiting' | 'inProgress' | 'finished';
  roomName: string;
}

export class Room {
  //创建游戏
  public static createGame(roomName: string): void {
    const gameState: RoomState = {
      players: [
        { name: 'Alice', hands: [], isLandlord: false, isReady: false }
      ],
      currentPlayerIndex: 0,
      roomState: 'waiting',
      roomName: roomName
    };
    localStorage.setItem(roomName, JSON.stringify(gameState));
  }

  //更新玩家状态
  public static updatePlayerStatus(roomName: string, playerName: string, isReady: boolean, isLandlord: boolean, hands: PokerCard[]): void {
    const gameState = Room.getRoomState(roomName);
    const player = gameState.players.find(p => p.name === playerName);
    if (player) {
      player.isReady = isReady;
      player.isLandlord = isLandlord;
      player.hands = hands;
      localStorage.setItem(roomName, JSON.stringify(gameState));
    }
  }
  //获取整个游戏状态
  public static getRoomState(roomName: string): RoomState {
    return JSON.parse(localStorage.getItem(roomName) || '{}') as RoomState;
  }
  //加入游戏
  public static joinGame(roomName: string): void {
    const gameState = Room.getRoomState(roomName);
    const playNum = gameState.players.length;
    if (playNum < 3) {
      if (playNum === 1) {
        gameState.players.push({ name: 'Bob', hands: [], isLandlord: false, isReady: false });
      }
      if (playNum === 2) {
        gameState.players.push({ name: 'Carol', hands: [], isLandlord: false, isReady: false });
      }
      localStorage.setItem(roomName, JSON.stringify(gameState));
    }
  }

}
