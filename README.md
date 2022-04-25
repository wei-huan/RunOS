# MyOS

## 介绍

把 task manager 独立出来作为 scheduler，仍然采取 roundrobin 方式，目前 suspend 函数如何保存上文很有问题，不知如何是好

suspend 改造完成

但压力测试未通过，怀疑是栈的页表问题，很有可能是溢出了
堆也有问题，内核堆泄漏了, 在测试stdin文件时，多次执行文件读取字符串堆就爆了，怀疑是异常输出流前没有回收资源

堆问题解决了，确实是buf爆了

页表问题页解决了，问题就是栈溢出，调度逻辑中shceduler无限嵌套，深度太大把栈搞崩了，暂时在scheduler时把栈清空，得过且过

k210的sbi还是选用rustsbi，rustsbi通过软件方式支持1.9.1版本的特权级架构，对k210更友好，其他功能也和Opensbi spec 1.0一样

今天是2022年4月10日
下一步：
继续丰富syscall，包括fork wait exec，思考多核如何实现，是否需要调整 4.12完成
多核有问题readline? 4.12完成
报名，完成测试
gdb 图形化窗口支持  4.12完成
fat32
shell改造   关机    关shell
slab缓存器
页面置换算法

今天是2022年4月18日
初赛测试程序能正常运行，1/5能跑通
fat32嫖来的能用了
shell改造成能关的了，但还不能关机
报名继续鸽
多核fork wait exec没发现太大问题，只是TCB会有重复借用问题，可能要改成Mutex
基于优先级的调度策略？
slab缓存器
页面置换算法

下一步：
需要fat32在sdcard上运行成功
深入理解Fat32,最好自己能写一个
调度逻辑不够RAII，需要改，同时要保留灵活性
继续丰富syscall
报名，提交部分代码
shell改造， 要能关机

今天是2022年4月24日
深入理解Fat32, 写了一半，规范太反人类，搁置
怎么wait和yield问题这么大
suspend_current_and_run_next 问题逐渐显现，要缕清，不然以后很麻烦
uname dup dup2 gettimeofday, times搞定

下一步：
完成和wait4相关的syscall
需要fat32在sdcard上运行成功
继续丰富syscall
报名，提交部分代码

今天是2022年4月25日
getppid搞定
