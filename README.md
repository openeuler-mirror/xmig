# Mnemosyne

#### 介绍
The rapid scaling of large language model (LLM) training clusters has made GPU errors the norm rather than the exception. Traditional error recovery methods such as periodic checkpointing introduce substantial overhead in both daily operations and recovery processes. 
While the most recent advances in just-in-time checkpointing reduces the overhead by eliminating the periodic checkpoint saving procedure and optimizing the recovery workflow, the intrinsic GPU context decoupling mechanism and global reinitialization of communication backend remain resource-intensive and slow. 

In this repo, we design Mnemosyne, a lightweight and fast error recovery framework for efficient LLM training. 
Mnemosyne first introduces a lightweight device proxy architecture optimized for fault recovery rather than general elasticity, reducing steady-state operational costs. 
Second, Mnemosyne designs a flexible collective communication library (CCL) that supports communication/calculation-free re-initialization as well as communicator scaling and adjustment, greatly accelerating the construction of collective communication.

The technical foundation of Mnemosyne lies in our well-designed techniques. 
First, the device proxy takes advantage of our automated code generation tools to boost our development workflow. Besides, all phases of its workflow are facilitated by efficient mechanisms, e.g., fused IPC and logging, unified ultra-fast handle mapping, state pulling with taken-over GPU memory management, etc. 
Second, the flexible CCL makes as many resources as possible persistent to reduce the rebuilding overhead. Specifically, for the reinitialization case, it fully leverages built bootstrap network as well as calculated topology and graph information to avoid repeated operations. Additionally, in the adjustment case, it conducts initialization operations in different scopes according to each rank’s original status.

#### 软件架构
软件架构说明


#### 安装教程

1.  xxxx
2.  xxxx
3.  xxxx

#### 使用说明

1.  xxxx
2.  xxxx
3.  xxxx

#### 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request


#### 特技

1.  使用 Readme\_XXX.md 来支持不同的语言，例如 Readme\_en.md, Readme\_zh.md
2.  Gitee 官方博客 [blog.gitee.com](https://blog.gitee.com)
3.  你可以 [https://gitee.com/explore](https://gitee.com/explore) 这个地址来了解 Gitee 上的优秀开源项目
4.  [GVP](https://gitee.com/gvp) 全称是 Gitee 最有价值开源项目，是综合评定出的优秀开源项目
5.  Gitee 官方提供的使用手册 [https://gitee.com/help](https://gitee.com/help)
6.  Gitee 封面人物是一档用来展示 Gitee 会员风采的栏目 [https://gitee.com/gitee-stars/](https://gitee.com/gitee-stars/)
