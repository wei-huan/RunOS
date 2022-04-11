# MyOS

## 介绍

把 task manager 独立出来作为 scheduler，仍然采取 roundrobin 方式，目前 suspend 函数如何保存上文很有问题，不知如何是好

suspend 改造完成

但压力测试未通过，怀疑是栈的页表问题，很有可能是溢出了
堆也有问题，内核堆泄漏了, 在测试stdin文件时，多次执行文件读取字符串堆就爆了，怀疑是异常输出流前没有回收资源

堆问题解决了

页表问题页解决了，问题就是栈溢出，调度逻辑中shceduler无限嵌套，深度太大把栈搞崩了，暂时在scheduler时把栈清空，得过且过

k210的sbi还是选用rustsbi，rustsbi通过软件方式支持1.9.1版本的特权级架构，对k210更友好，其他功能也和Opensbi spec 1.0一样

下一步：
继续丰富syscall，包括fork wait exec，思考多核如何实现，是否需要调整

报名，完成测试

slab缓存器加上

gdb 图形化窗口支持

页面置换算法

