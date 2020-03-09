https://learnblockchain.cn/article/714

展开宏
要理解宏背后做的工作，最直接的方式当然就是看宏自身是怎么写的。但是平心而论，Substrate 编写这块的作者虽然有一些滥用宏，但是他的技巧是十分高超，生成宏这部分的代码量都十分庞大。若不是对宏系统十分熟悉（因为如果只是写简单的宏理解起来不困难，但是若不常写，只是看宏的话那些$替换符会很别扭，思维也不容易把这些联系起来），那么硬生生去读宏的写法会相当困难。

因此若只是为了理解宏最后干了什么事情的话，使用宏展开比去理解宏的写法好得多。

因此本文介绍在 Runtime 中宏展开及一些相应技巧。

首先先要明确一个前提，由于之前的介绍，我们应该能理解对于 Runtime 而言，native 和 wasm 应该在大部分情况下是同一份代码。因此我们展开宏一般情况下只针对 native 展开。很特殊及稀少的情况下才可能需要 wasm 展开。那么在展开 wasm 的时候请依据之前的文章添加上相应的 feature 开关。

展开宏笔者在这里介绍是对于crate维度，不针对xxx.rs维度。因为只编译xxx.rs难度很大，而且在很多情况下反而不太方便。

例如如果想要 node 项目中的 runtime 的宏，那么首先切换到相应的crate目录下：

cd bin/node/runtime/
然后使用carge的宏展开命令：

cargo rustc -- -Z unstable-options --pretty=expanded > runtime.rs
由于目前 Substrate 已经支持 stable 的 rust 了，所以这里展开没必要用 nightly。如果需要 nightly，那么在cargo后面加上+nightly