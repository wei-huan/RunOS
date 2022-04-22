#![allow(unused)]
mod block_dev;
mod cluster_cache;
mod fat_table;
mod sector_cache;

use block_dev::BlockDevice;
use sector_cache::{SectorCache, get_sector_cache};

const BLOCK_SZ: usize = 512;
const SEC_SZ: usize = BLOCK_SZ;
const SECS_PER_CLU: usize = 16;
const CLUS_SZ: usize = SEC_SZ * SECS_PER_CLU;
const RESERVE_SECS: usize = 32;
const FAT_TABLE_SECS: usize = 15184;
const DBR_START_SEC: usize = 0;
const FAT_TABLE_START_SEC: usize = DBR_START_SEC + RESERVE_SECS;
const DATA_START_SEC: usize = FAT_TABLE_START_SEC + FAT_TABLE_SECS * 2;

// 扇区缓冲区长度
const SECTOR_CACHE_SIZE: usize = 8;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
