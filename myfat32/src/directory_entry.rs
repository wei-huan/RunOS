enum FileDirectoryEntry {
    LongDirectoryEntry(LongDirectoryEntry),
    ShortDirectoryEntry(ShortDirectoryEntry),
    ScrollEntry(ScrollEntry),
    PresentEntry(PresentEntry),
    LastEntry(LastEntry),
}

struct LongDirectoryEntry {
    // use Unicode !!!
    // 如果是该文件的最后一个长文件名目录项，
    // 则将该目录项的序号与 0x40 进行“或（OR）运算”的结果写入该位置。
    // 长文件名要有\0
    attribute: u8,   // 删除时为0xE5
    name1: [u8; 10], // 5characters
    flag: u8,        // should be 0x0F
    os_reserved: u8, //
    check_sum: u8,
    name2: [u8; 12], // 6characters
    zero: [u8; 2],
    name3: [u8; 4], // 2characters
}

struct ShortDirectoryEntry {
    name: [u8; 8], // 删除时第0位为0xE5，未使用时为0x00. 有多余可以用0x20填充
    extension: [u8; 3],
    attribute: u8, //可以用于判断是目录还是文件
    os_reserved: u8,
    creation_tenths: u8, //精确到0.1s
    creation_time: u16,
    creation_date: u16,
    last_acc_date: u16,
    cluster_high: u16,
    modification_time: u16,
    modification_date: u16,
    cluster_low: u16,
    size: u32,
}

struct ScrollEntry {}

struct PresentEntry {}

struct LastEntry {}
