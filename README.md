# MyOS



## 介绍
在裸机运行环境下尝试 k210 多核运行



## 现状
1，第二个核有相关输出

2，但有串口资源竞争

3，有时会触发 rustsbi 的 panic

```
[rustsbi-panic] hart 0 panicked at 'rustsbi-qemu: machine soft interrupt with no hart state monitor command', rustsbi-qemu/src/execute.rs:71:25
```





