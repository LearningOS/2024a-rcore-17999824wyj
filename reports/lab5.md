# Lab 5. Report

@Author: Wang Yijia (abcd1234)  
@Email: abcd1234dbren@yeah.net

## 我实现的功能

1. 为进程添加资源列表，主要是已分配的资源和还需要的资源。
2. 为进程添加 using 结构体，用于方便的管理是否启用死锁检测。
3. 按照说明，完成对信号量的银行家算法。
4. 对于互斥锁的加锁和解锁，只需要验证是否资源已加锁，即可判断是否死锁。

## 问答题

### 在我们的多线程实现中，当主线程 (即 0 号线程) 退出时，视为整个进程退出，此时需要结束该进程管理的所有线程并回收其资源。

- 需要回收的资源有哪些？  
  分配的地址空间要回收，打开的文件描述符可以采用直接关闭的方式，同时其它的资源描述符的空间需要回收。我没有手动回收，但 rust 应该会自行回收吧，这点我确实不太确定。

- 其他线程的 TaskControlBlock 可能在哪些位置被引用，分别是否需要回收，为什么？  
  可能位于其它的处理器的执行队列中(多核)、可能位于资源的等待队列中。需要回收，如果不回收，那么一旦执行到“已死”的线程，就会产生异常。  
  有一个比较具象化的例子：假设一个子线程位于另一个处理器的执行队列，则在此线程被执行完毕后，如果主线程被回收了而其没有被主动清除，那它就会一直停留在其他的队列里，当下一次处理器执行到它时，就会产生异常。而如果为了此异常单独做一个处理，那就不如直接主动回收掉了。

### 两种 mutex 实现的区别

Mutex1 的实现在解锁过程中先解锁，然后再尝试唤醒。这样做的好处是如果唤醒了等待队列中的线程，它们可以立即获得锁，而不需要等待当前线程完全退出。

Mutex2 的实现则在尝试唤醒后，才将锁解掉。这样做的好处是如果等待队列中没有线程，就不会额外进行锁状态变更操作。但是，如果等待队列中有线程需要唤醒，这种实现可能会导致等待线程在等待当前线程完全退出后才能获得锁，从而可能会增加线程间切换的时间。

Mutex1 的实现更倾向于提高系统的响应性能，而 Mutex2 的实现则更注重减少不必要的锁状态变更操作。

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：  
   _无_

2. 此外，我也参考了以下资料，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：  
   _无_

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 看法

不知是我不够细心还是新版指导书里没写，我因为没有在 ch8 实现 sys_time 的系统调用而卡了很久。我找了相当长的时间，一直以为是不是我的算法实现有问题，结果后来发现是 sys_call 没实现！？建议加个提醒！！！