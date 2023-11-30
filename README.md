
## 基本信息

- 官方链接：https://github.com/parity-asia/hackathon-2023-winter
- 项目定位：波卡上发一条专门的GameFi链，专用于信息不对称游戏的创建，示例游戏为扑克游戏。
- 技术路线：利用substrate框架，创建pallet实现ZK的验证器，开发扑克游戏的电路逻辑，利用前端页面进行ZK证明，并提交交易上链完成游戏。
- 截止日期：2023年12月25日


## 工作分解：

- 节点：利用substrate框架，开发pallet，实现链上ZK的Verify
- 前端：扑克游戏的交互界面
- 合约：扑克游戏的公开规则验证
- 电路：开发ZK电路，实现扑克游戏的验证逻辑
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
- https://www.youtube.com/watch?v=KGs3A3GMcuw

## 问答

Q: 为什么要做？为什么值得做？

A: 在传统游戏环境中，服务器可能导致作弊行为，从而影响游戏的公平性。而借助于零知识证明这一技术，有效地确保了游戏的公平性。