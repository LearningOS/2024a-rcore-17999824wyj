# Lab 1. Report

@Author: Wang Yijia (abcd1234)  
@Email: abcd1234dbren@yeah.net

## 我做到的功能

1. 在 TaskControlBlock 中增加了两个字段，用于记录系统调用的次数和记录任务开始运行的时间，其中时间是 `Option<usize>`，若 `None`，则说明没产生过系统调用。
2. 为 TaskManager 增加了两个方法，分别用于为当前 task 增加系统调用的记录和返回当前 task 的 taskinfo。
3. 为 TaskInfo 增加 new 方法，更好的实现信息隔离。
4. 为 task 包，增加了一些接口，用于调用 TaskManager 的两个方法。
5. 为系统调用实现了“拿到当前任务的 TaskInfo”。

## 简答题

### 1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b*bad*\*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

rcore 会报无效指令错误，然后内核将程序 kill，回收内存

```plaintext
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```

[rustsbi] RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0

### 2. 深入理解 trap.S 中两个函数 **alltraps 和 **restore 的作用，并回答如下问题:

#### 1. L40：刚进入 **restore 时，a0 代表了什么值。请指出 **restore 的两种使用情景。

因为在\_\_alltraps 的最后，通过"mv a0, sp"的操作，将 sp 的值 copy 到 a0 里，所以 a0 代表的是 sp 的值。而 sp 是当前任务的内核栈，所以 a0 代表的是当前任务的内核栈的栈顶。

\_\_restore 的两种使用情景：

- 当出现了异常，操作系统会进入内核态，处理此异常，处理完毕后，可能通过此函数回到当前任务的用户态。
- 当系统调用结束后，操作系统会通过此函数回到用户态，继续执行用户态的代码。

#### 2. L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

```asm
ld t0, 32*8(sp)
ld t1, 33*8(sp)
ld t2, 2*8(sp)
csrw sstatus, t0
csrw sepc, t1
csrw sscratch, t2
```

L43-L48 这几行汇编代码特殊处理了 t0、t1 和 t2 寄存器。这些寄存器保存了处理器状态和用户态的上下文信息：

- t0 寄存器保存了处理器状态（sstatus 寄存器）的值，在进入用户态时，需要将处理器状态设置为用户态，以允许用户程序执行。
- t1 寄存器保存了异常程序计数器（sepc 寄存器）的值，这是用户程序被中断时应该返回的地址。
- t2 寄存器保存了用户栈指针（sscratch 寄存器）的值，用于恢复到用户态后正确设置栈指针。

#### 3. L50-L56：为何跳过了 x2 和 x4？

```asm
ld x1, 1*8(sp)
ld x3, 3*8(sp)
.set n, 5
.rept 27
   LOAD_GP %n
   .set n, n+1
.endr
```

x2 寄存器通常用作栈指针，但是因为在 \_\_restore 函数中，已经通过 “csrrw sp, sscratch, sp” 指令将栈指针设置为用户栈，就不需要再额外保存和恢复 x2。  
x4 寄存器通常用作线程指针，但是在目前，我们还没有线程，所以不使用 x4 寄存器，因此没有保存和恢复的必要。

#### 4. L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？

```asm
csrrw sp, sscratch, sp
```

sp 寄存器中保存着用户栈的地址，而 sscratch 寄存器中保存着内核栈的地址。

#### 5. \_\_restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

状态切换发生在 sret 指令处。因为当前位于内核态，如果调用 sret 指令，就可以使操作系统从内核态返回用户态。

#### 6. L13：该指令之后，sp 和 sscratch 中的值分别有什么意义？

```asm
csrrw sp, sscratch, sp
```

sp 寄存器中保存着内核栈的地址，而 sscratch 寄存器中保存着用户栈的地址。

#### 7. 从 U 态进入 S 态是哪一条指令发生的？

在汇编代码（第 13 行）：

```asm
csrrw sp, sscratch, sp
```

这部分代码通过值交换，将栈指针从 U 态的栈重定位至 S 态的栈。

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：  
   _无_

注：我曾于交流群群友们，交流过"rust-anaylzer"插件的报错问题，后来发现是因为我没有好好读指导书。

2. 此外，我也参考了以下资料，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：  
   _无_

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 看法

建议在某些地方提一下，二刷同学也要重新看指导书。。。这个可能算一个小坑吧。