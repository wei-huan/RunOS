# MyOS

## 介绍
在裸机运行环境下尝试 k210 多核运行


## 进度
2022.2.24
1，第二个核有相关输出

2，但有串口资源竞争

3，有时会触发 rustsbi 的 panic

```
[rustsbi-panic] hart 0 panicked at 'rustsbi-qemu: machine soft interrupt with no hart state monitor command', rustsbi-qemu/src/execute.rs:71:25
```

2022.2.25
1，sbi换成opensbi之后没有异常，但输出仍然诡异，说明自己对多核还是不够熟悉

2，没有资源竞争，只是程序执行流非常奇怪。

3, 现在输出勉强能看了，但还是很狼狈，显然需要实现自旋锁

4，完成了启动栈的分配，

希望能完成基于多核的进程管理，需要先完成内核栈的分配和锁机制

fence指令相当于gcc的__sync_synchronize

2022.2.26
草草草草草草草 sbi2.0的ecall换api了，草，麻了
