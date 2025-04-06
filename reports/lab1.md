# lab1(ch3)

## 我实现的功能

实现`trace`系统调用，`trace_request`为0和1的情况直接操作指针读写；仿照`TaskManager`的方式创建了一个全局变量`SyscallCount`结构体，内部是`UPSafeCell<[BTreeMap<usize, usize>; MAX_APP_NUM]>`，在每次系统调用中对`current_task`的对应的系统调用号的调用次数加一，`trace_request`为2的情况在 `SyscallCount`中查找。

## 问答题

### 1.

报错为：
```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```
* `bad_address`: 访问了地址为0x0处的内存，0x0为空指针，触发PageFault
* `bad_instructions`: 在U特权级使用sret指令，权限不够，触发IllegalInstruction
* `bad_register`: 在U特权级使用csrr指令访问sstatus寄存器，权限不够，触发IllegalInstruction

我使用的sbi版本是 0.3.0-alpha.2

### 2.

#### 1.

刚进入 __restore 时，sp 代表了内核栈中的一个位置。

第一种场景是对Trap上下文的恢复，上面的`__alltraps`函数调用`trap_handler`后没有返回，直接到了`__restore`函数，恢复应用程序的寄存器后返回到用户态继续运行。

第二种场景是任务切换，每个任务的任务上下文初始化的时候将存储`ra`寄存器的变量设置为`__restore`的地址，在`__switch`函数中，会恢复上下文中保存的`ra`，因此`__switch`函数返回后会跳转到`__restore`继续执行，返回到用户态继续运行。

#### 2.

特殊处理了`sstatus`, `sepc`, `sscratch`

* `status`: 管理中断和异常的控制寄存器。包括中断使能和特权级，正确设置才能在用户态响应trap。
* `sepc`: 保存发生异常时的程序计数器的值。用于在返回用户态后找到继续执行的位置。
* `sscratch`: 用于临时存储数据的寄存器。这里储存了内核栈的地址，下次trap时用得到。

#### 3.

`x2`是`sp`寄存器，已经保存过了，`x4`是`tp`寄存器，一般用不到。

#### 4.

`sp`的值在这之后指向用户栈，`sscratch`在这之后指向内核栈。

#### 5.

sret指令，sstatus中的字段会保存当前的特权级，从S特权级返回就是回到用户态。

#### 6.

`sp`的值在这之后指向内核栈，`sscratch`在这之后指向用户栈。

#### 7.

用户中导致trap的指令，可能是ecall，也可能是执行了权限不够的指令，或者是出错了。

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
