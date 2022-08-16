## bug
1. fork 之后子进程 heap 段 PageFault, 原因是 sys_brk 系统调用中忘了修改 VPNRange 范围, 在f ork 时没有 copy 到, mmap段也有这个bug, 已改

2. lmbench_all 任何一个测试都会卡在

```
[    4.195][ DEBUG ][HART 0][kernel::syscall::fs] sys_read fd: 3, buf: 0x0000000000111650, len: 1
```

这个位置, 非常尴尬, 怀疑是 sys_read sys_write sys_pipe sys_pselect6 这几个系统调用有东西没有实现
对比 UltraOS 发现 sys_pselect 要实现检测文件当前是否可读可写, 这个时候管道 fd 3 没有东西可读, 所以就一直卡在这, 本来sys_pselect6 应该返回 fd 3 readfds not ready, 但是还没有具体检测, 需要实现 当然目前来说可以默认普通文件是一直准备好读写的, 只是抽象文件需要实现. 实现完之后问题解决

3. 新 bug 是没有实现 sys_settitimer 系统调用, 参考 linux 的(SYSCALL_DEFINE3(setitimer, int, which, struct __kernel_old_itimerval __user *, value, struct __kernel_old_itimerval __user *, ovalue))实现

4. 缺乏 dev/null 和 dev/zero的实现 无法完成 lmbench_all lat_syscall -P 1 read 和 lmbench_all lat_syscall -P 1 write, 中途实现 dev 时发现 sys_open_at 也有严重的问题, 需要调整, 实现 /dev/null和/dev/zero 之后未优化QEMU成绩为
   
```
>> lmbench_all lat_syscall -P 1 null
Simple syscall: 29.6713 microseconds
>> lmbench_all lat_syscall -P 1 read
Simple read: 40.3337 microseconds
>> lmbench_all lat_syscall -P 1 write
Simple write: 38.3691 microseconds
```

同时 stat fstat 和 sig install 测试也都通过了

5. 很多 lmbench 测试会 clone 3个进程进行测试, 但没有 exec 新内容, 所以白白复制了 3 次地址空间, 非常浪费, 在 K210 平台还会耗尽 Pages 造成崩溃 所以实现写时复制(Copy on Write)刻不容缓了

6. 测试过程怎么有时快有时慢, 非常奇怪, 原来是 syscall pselect6 实现时理解有点问题, 有个小bug, 改了之后耗时还短了, 乐

7. 还是必须实现信号机制, 只能说透心凉吧

8. sys_open_at 的 OpenFlags 有 bug, 改正之后 Select on 100 fd's: 17268.5681 microseconds 好了

9. 目前内存空间还是很紧张, 经常物理页帧不够用, 发现内核堆竟然膨胀到了了4.2MB, 原因就是 sys_exec 加载 elf 时用了堆空间, lmbench 大小 1.1 MB, 根据伙伴系统的原理那么就需要 2 MB内存去加载, 显然对于 K210 不合理, 不如直接 map 到 frame 中, 可以砍去至少 2 MB 的内核堆  