# MyOS

## 介绍

把 task manager 独立出来作为 scheduler，仍然采取 roundrobin 方式，目前 suspend 函数如何保存上文很有问题，不知如何是好

suspend 改造完成

但压力测试未通过，怀疑是栈的页表问题，很有可能是溢出了
堆也有问题，内核堆泄漏了, 在测试stdin文件时，多次执行文件读取字符串堆就爆了，怀疑是异常输出流前没有回收资源

堆问题解决了，确实是buf爆了

页表问题页解决了，问题就是栈溢出，调度逻辑中shceduler和supervisor timer无限嵌套，深度太大把栈搞崩了，暂时在scheduler时把栈清空，得过且过

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

今天是2022年4月26日
suspend_current_and_run_next 改造完成, 方法比较不错, 和原来的调度逻辑不冲突
S态的 supervisor timer 还是和多核有冲突
wait waitpid exit yield搞定
还差将近一半的syscall没做完(14个任务)

下一步：
supervisor timer bug改, 暂时猜测是 task 的 kernel_stack 崩了，要么 user_trap 关中断，
要么 supervisor timer 到 schedule 自己换栈
需要 fat32 在 sdcard 上运行成功
继续丰富syscall
报名，提交部分代码

今天是2022年4月27日
理清任务的全部执行流，supervisor timer很奇怪, 和多核冲突，原因未知
还需要完成信号量，线程，管道，动态内存分配，文件系统，月前任务艰巨
getcwd，fstat完成
还差将近一半的syscall没做完(12个任务)

下一步：
理清任务的全部执行流，改 supervisor timer 和多核冲突 bug
需要 fat32 在 sdcard 上运行成功
盖章，报名，改队名
继续丰富syscall

今天是2022年4月28日
昨晚玩了一天
改了队名
supervisor timer 和多核冲突 bug 原因找到了, user_trap_handler 后又时钟中
断进入 kernel_trap_handler，无语，要在 user_trap 禁时间中断
上面的bug改掉了，舒坦！！！
pipe, getdents, chdir完成
还差 1 / 3 的syscall没做完(9个任务)
mkdir_, openat, clone 都有些问题，需要解决

下一步：
盖章，改报名
备份保存正常版本
需要 fat32 在 sdcard 上运行成功
尝试提交
继续丰富syscall

今天是2022年4月29日
睡到12点，颓废煞笔
提交了扫描文件，还有邱老师的教师证文件没提交
gitlab账号有问题，没法提交
备份保存正常版本完成
fat32 在 sdcard 上运行成功
openat, mkdir_完成

下一步：
提交邱老师的教师证文件