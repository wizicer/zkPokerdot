官方链接：https://github.com/parity-asia/hackathon-2023-winter

## 基本信息

- 项目定位：波卡上发一条专门的GameFi链，专用于信息不对称游戏的创建，示例游戏为扑克游戏。
- 技术路线：利用substrate框架，创建pallet实现ZK的验证器，开发扑克游戏的电路逻辑，利用前端页面进行ZK证明，并提交交易上链完成游戏。
- 截止日期：2023年12月25日


## 工作分解：

- 节点：利用substrate框架，开发pallet，实现链上ZK的Verify
- 电路：开发ZK电路，实现扑克游戏的验证逻辑
- 前端：扑克游戏的交互界面
- 初始化：可验证秘密洗牌(Verifiable Secret Shuffling)


## 参考链接

- https://github.com/Zkvers/substrate-zk
- https://hackmd.io/@nmohnblatt/SJKJfVqzq
- https://medium.com/coinmonks/zk-poker-a-simple-zk-snark-circuit-8ec8d0c5ee52
- https://geometry.xyz/notebook/mental-poker-in-the-age-of-snarks-part-1
- https://geometry.xyz/notebook/mental-poker-in-the-age-of-snarks-part-2
- https://github.com/geometryresearch/mental-poker/
- https://zkholdem.xyz/wp-content/themes/zkholdem-theme/zkshuffle.pdf
- https://docs.manta.network/docs/guides/zkHoldem/About

## 问答

Q: 为什么要做？

A: 传统游戏中，通过服务器，容易出现作弊，导致游戏不公平，但利用ZK，可以解决游戏公平性的问题。