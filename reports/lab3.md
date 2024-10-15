# Lab 3. Report

@Author: Wang Yijia (abcd1234)  
@Email: abcd1234dbren@yeah.net

## 我实现的功能

1. 在 ch4 基础上新建分支，再使用 `git merge --squash` 合并全部历史，然后用 `cherry-pick` 合并到 ch5。
2. 照着 fork 和 exec 仿写，完成了 spawn。
3. 完成了 stride 调度算法。

## 问答题

### 实际是 p1 执行吗？为什么？

不是，因为 260 的二进制数是 100000100。高位溢出后，p2 的 stride 又小于 p1 了，所以实际是 p2 继续执行。

### 证明 STRIDE_MAX – STRIDE_MIN <= BigStride / 2

因为一位二进制就代表乘二，这样能够在溢出时，仍然是算法正常。

### 实现 partial_cmp

```rust
// 我的思路是，列举特殊情况，直接匹配，这样最快！
impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 > other.0 && self.0 - other.0 <= BigStride / 2 {
            return Some(Ordering::Greater)
        } else if self.0 > other.0 && self.0 - other.0 > BigStride / 2 {
            return Some(Ordering::Less)
        } else if self.0 < other.0 && other.0 - self.0 <= BigStride / 2 {
            return Some(Ordering::Less)
        } else if self.0 < other.0 && other.0 - self.0 > BigStride / 2 {
            return Some(Ordering::Greater)
        }
    }
}
```

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：  
   _无_

注：我在使用 `git cherry-pick` 工具时遇到了一些问题，主要是 `cherry-pick` 只能合一个分支，但我是一点点来的，如果分别来的话太慢，`rebase` 效果也不好，`merge` 因为无相同历史导致无法使用。后续请教了东北大学软件学院张引老师，他提出了一种方法加快了我解决代码合并的问题。

2. 此外，我也参考了以下资料，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：  
   _无_

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 看法

无
