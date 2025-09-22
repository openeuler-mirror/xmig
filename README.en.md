# Mnemosyne

#### Description
The rapid scaling of large language model (LLM) training clusters has made GPU errors the norm rather than the exception. Traditional error recovery methods such as periodic checkpointing introduce substantial overhead in both daily operations and recovery processes. 
While the most recent advances in just-in-time checkpointing reduces the overhead by eliminating the periodic checkpoint saving procedure and optimizing the recovery workflow, the intrinsic GPU context decoupling mechanism and global reinitialization of communication backend remain resource-intensive and slow. 

In this repo, we design Mnemosyne, a lightweight and fast error recovery framework for efficient LLM training. 
Mnemosyne first introduces a lightweight device proxy architecture optimized for fault recovery rather than general elasticity, reducing steady-state operational costs. 
Second, Mnemosyne designs a flexible collective communication library (CCL) that supports communication/calculation-free re-initialization as well as communicator scaling and adjustment, greatly accelerating the construction of collective communication.

The technical foundation of Mnemosyne lies in our well-designed techniques. 
First, the device proxy takes advantage of our automated code generation tools to boost our development workflow. Besides, all phases of its workflow are facilitated by efficient mechanisms, e.g., fused IPC and logging, unified ultra-fast handle mapping, state pulling with taken-over GPU memory management, etc. 
Second, the flexible CCL makes as many resources as possible persistent to reduce the rebuilding overhead. Specifically, for the reinitialization case, it fully leverages built bootstrap network as well as calculated topology and graph information to avoid repeated operations. Additionally, in the adjustment case, it conducts initialization operations in different scopes according to each rank’s original status.

#### Software Architecture
Software architecture description

#### Installation

1.  xxxx
2.  xxxx
3.  xxxx

#### Instructions

1.  xxxx
2.  xxxx
3.  xxxx

#### Contribution

1.  Fork the repository
2.  Create Feat_xxx branch
3.  Commit your code
4.  Create Pull Request


#### Gitee Feature

1.  You can use Readme\_XXX.md to support different languages, such as Readme\_en.md, Readme\_zh.md
2.  Gitee blog [blog.gitee.com](https://blog.gitee.com)
3.  Explore open source project [https://gitee.com/explore](https://gitee.com/explore)
4.  The most valuable open source project [GVP](https://gitee.com/gvp)
5.  The manual of Gitee [https://gitee.com/help](https://gitee.com/help)
6.  The most popular members  [https://gitee.com/gitee-stars/](https://gitee.com/gitee-stars/)
