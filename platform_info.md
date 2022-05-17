qemu5.0 和 k210 的特权级版本不同，qemu5.0是rv.10 k210是rv1.9
qemu5.0 和 k210 的设备树文件内容不同
rustsbi 多核启动逻辑和 opensbi 的不太一样，boot_hart_id 和 启动跳转地址都有些区别

os_main
fdt_get_timerfreq
boot_all_harts
以上3处要改成对应版本
