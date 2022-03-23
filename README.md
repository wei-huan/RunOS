# MyOS

## 介绍

把 task manager 独立出来作为 scheduler，仍然采取 roundrobin 方式，目前 suspend 函数如何保存上文很有问题，不知如何是好

suspend 改造完成

但压力测试未通过，怀疑是栈的页表问题，很有可能是溢出了

