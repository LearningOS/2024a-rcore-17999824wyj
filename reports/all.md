---
title: abcd1234-rust-os
date: 2024-10-15 18:32:19
categories:
  - rust-stage-1
tags:
  - author:17999824wyj
  - repo:https://github.com/LearningOS/2024a-rcore-17999824wyj
  - noob
  - nickname:abcd1234
  - email:abcd1234dbren@yeah.net
---

# Rust OS 一阶段学习总结

@FinishTime: 2024-09-19 00:35:51

## 个人背景概述

我是一名软件工程专业的大三本科生，曾参与过 2023 年秋冬季训练营和 2024 年春夏季训练营。并且在 2024 季春夏训练营中，取得了通过的成绩。  
今年的秋冬季训练营，我想冲一冲优秀，多做一些项目，多为国内操作系统和开源做出贡献。

## 今年一阶段学习时间表

- 2024-09-18 大约 3.5 小时，重新完成 rustlings 的官方基础习题(前 94 题)。之后，又用了大约 2 小时，完成了后续的习题(95-110)。

## 一阶段学习内容概述

我按照[“rust 语言圣经”](https://course.rs/about-book.html)上的讲解顺序，复习之前的知识点。  
由于我在那段时间之前，在参加 InfiniTensor 的 AI 训练营，所以 rust 的基础语法等根本没有落下，这使得我在完成 rustlings 的习题时，几乎没有遇到什么问题。

只有到后续的习题中，我遇到了一些问题，主要是和 `智能指针` 有关的内容。这些部分我一直不是很理解的。但是因为我的经验，我还是能写出代码的。

## 总结

在第一阶段的学习中，我巩固了我所掌握的 rust 基础，我更是深深的意识到：`Talk is cheap, show me the code!`。实践是最好的老师！

# Rust rCore 二阶段学习总结

@FinishTime: 2024-10-15 18:44:32

## 二阶段学习时间表

- 2024-10-08 开始实验，轻车熟路，一天晚上完事了大部分的 ch3。
- 2024-10-09 凌晨 ch3 debug 完毕，下午进行提交完毕。
- 2024-10-09 晚上 ch4 提交完毕。
- 2024-10-11 ch5 提交完毕。
- 2024-10-13 ch6 提交完毕。
- 2024-10-15 ch8 提交完毕。撰写报告完毕。

## 问题复盘

通过上面的学习时间表可以看出，我虽然是第二次做大实验，但仍然每一部分都需要 1 到 2 天时间。虽然不是将大段时间全部投入，但可能也是比较慢了。  
以下是我实验过程中的问题复盘。

### ch3: 本地测例初始化问题

在 ch3 里，我遇到了两个问题，第一个是本地测例的初始化问题。

由于我是第二次参与训练营，所以在实验时，我就直接去看指导书的习题部分，在尝试开始实现时，在 rust-anaylzer 的使用上时发生了错误，插件因为找不到某些依赖库，拒绝提供服务。

我去查看了插件的报错信息，发现找不到 `user` 目录。而我在源代码仓库，也确实没看到源代码里有 `user` 目录。后续，我在自行尝试解决无果后，去微信群问了助教。得到的回答是 `看文档`。然后我去重新看了文档，找到了一个下载 `user` 的方法。这个问题是由于我的粗心导致的。

### ch3: 时间初始化问题

ch3 的第二个问题是，os 时间初始化问题。

在我实现的 os 里，在时间初始化，采用的是 `直接在任务加载时便初始化为当前系统时间`。这显然是错误的。但它的报错并不友好。

此问题的表现为：在直接指定 `BASE=1` 执行测试用例时，能够通过测试用例。指定 `BASE=0` 时，也能够通过测试。但指定 `BASE=2` 时，竟然不能通过测试！？同时，这种情况下，qemu 会出现“卡死”，让人一直等下去。

我通过细细阅读报错，发现 qemu 卡死是因为有一个用户测例 panic 导致的。经过分析的测例执行逻辑为：

```plaintext
我分析，ch3里，qemu在跑测试用例的时候，qemu的退出机制是，当全部测试用例成功时，才会退出。

如果有测例没过，他会接着执行别的测试用例，但最后全部测试用例执行完后，就不会退出了，还没有信息提示。
```

能够定位位置，之后的工作就要简单多了。我发现了问题所在，然后将时间改为 `Option` 类型，以 `None` 来初始化，在为某任务进行 sys_call 计数时进行判断，如果为 `None`，则进行初始化。这样，问题就解决了。

后续，我看到交流群里有个同学，和我遇到了一样的问题，于是，我也帮助了他。

### ch6: 又一次卡了一小点

在 ch6 时，平心而论，要求实现的逻辑并不是很复杂，只是需要层层的传递，最后由“easy-fs”进行实际干活即可。但我再次遇到了一个极为恶心的问题。

在春夏季训练营，我曾经遇到了“File is shorter than the first ELf header part”的问题，翻译过来就是，文件比文件描述符还短。我发现，只有 ch5_spawn0，也就是 helloworld 的测例出现了这个问题。

而这一次，我又遇到了这个问题。然后，我通过微信群的聊天记录，找到了当时一位热心同学给我的解答，即，此情况可能是由于 rust 的智能优化导致的，只需要把一个位置，由 `[u8; 512]` 改为一个 Vec 即可。

还好我留有聊天记录，再次感谢当时的那位同学！

### ch8: 系统时间系统调用未实现

来到了 ch8，我没有看到指导书文档里，写着的 “除 sys_get_time 以外”，所以，我就没有合并我之前的代码。但问题是，在我实现了 ch8 的内容后，进行测例，有两个始终过不去，就是那两个死锁相关的。而且，它没有报错提醒。这使我很懵，是不是我的银行家算法写的有问题？

在一次又一次地尝试为银行家算法进行 debug 后，我发现，好像我的算法没问题啊。于是，我又一次详细阅读测试代码，发现：这里面有个`sleep(500)`，这是调了哪个 sys_call？我查了，发现是获得系统时间的那个。但我没有合并之前的代码，导致其会进入死循环。由此，我才改好了问题。

## 总结

第二次来 rCore，感觉熟能生巧了一些，之前很多不太理解的部分，已经慢慢构不成威胁了。果然，实践能够带动理论！但我也因为自己的粗心，导致了一些不必要的麻烦。这是需要我进行反省的。

## 展望

这一次的三阶段，将会涉及到“组件化”。希望我能尽快完成相关内容，及早进入后续项目内容的学习！希望能够冲一冲优秀！