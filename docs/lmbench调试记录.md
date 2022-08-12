## bug
1. fork 之后子进程 heap 段 PageFault, 原因是 sys_brk 系统调用中忘了修改 VPNRange 范围, 在f ork 时没有 copy 到, mmap段也有这个bug, 已改

2. lmbench_all 任何一个测试都会卡在

```
[    4.195][ DEBUG ][HART 0][kernel::syscall::fs] sys_read fd: 3, buf: 0x0000000000111650, len: 1
```

这个位置, 非常尴尬, 怀疑是 sys_read sys_write sys_pipe sys_pselect6 这几个系统调用有东西没有实现
对比 UltraOS 发现 sys_pselect 要实现检测文件当前是否可读可写, 这个时候管道 fd 3 没有东西可读, 所以就一直卡在这, 本来sys_pselect6 应该返回 fd 3 readfds not ready, 但是还没有具体检测, 需要实现 当然目前来说可以默认普通文件是一直准备好读写的, 只是抽象文件需要实现. 实现完之后问题解决

3. 新 bug 是没有实现 sys_settitimer 系统调用, 参考 linux 的(SYSCALL_DEFINE3(setitimer, int, which, struct __kernel_old_itimerval __user *, value, struct __kernel_old_itimerval __user *, ovalue))实现

4. 缺乏 dev/null 和 dev/zero的实现 无法完成 lmbench_all lat_syscall -P 1 read 和 lmbench_all lat_syscall -P 1 write, 中途实现 dev 时发现 sys_open_at 也有严重的问题, 调整

