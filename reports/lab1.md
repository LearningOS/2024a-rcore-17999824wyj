# Lab 1. Report

@Author: Wang Yijia (abcd1234)  
@Email: abcd1234dbren@yeah.net

## 我做到的功能

1. 在 TaskControlBlock 中增加了两个字段，用于记录系统调用的次数和记录任务开始运行的时间。
2. 为 TaskManager 增加了两个方法，分别用于为当前 task 增加系统调用的记录和返回当前 task 的 taskinfo。
3. 为 TaskInfo 增加 new 方法，更好的实现信息隔离。
4. 为 task 包，增加了一些接口，用于调用 TaskManager 的两个方法。
5. 为系统调用实现了“拿到当前任务的 TaskInfo”。

## 简答题

### 1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

通过在 `build.py` 的第16行后添加下列内容来进行测试：
```python
apps = ["ch2b_bad_address.rs", "ch2b_bad_register.rs", "ch2b_bad_instructions.rs"]
```

rcore会报无效指令错误，然后内核将程序kill，回收内存
```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003ac, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```


## 荣誉准则
