/// 块缓存层，主要用于 FAT32 的保留扇区和 FAT 表
use super::{BlockDevice, BLOCK_SZ, SECTOR_CACHE_SIZE};
use lazy_static::*;
use spin::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;

pub struct BlockCache {
    cache: [u8; BLOCK_SZ],
    sector_id: usize,
    block_dev: Option<Arc<dyn BlockDevice>>, // Arc + dyn 实现 BlockDevice Trait 的动态分发
    modified: bool,
}

impl BlockCache {
    pub fn new(sector_id: usize, block_dev: Arc<dyn BlockDevice>) -> Self {
        let mut cache = [0; BLOCK_SZ];
        block_dev.read_block(sector_id, &mut cache);
        Self {
            cache,
            sector_id,
            modified: false,
            block_dev: Some(block_dev),
        }
    }
    pub fn get_cache_ref(&self) -> &[u8; BLOCK_SZ] {
        &self.cache
    }
    pub fn get_cache_mut(&mut self) -> &mut [u8; BLOCK_SZ] {
        &mut self.cache
    }
    pub fn get_ref<T>(&self, offset: usize) -> &T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SZ);
        unsafe {
            &*((&self.cache[offset..offset + type_size]).as_ptr() as *const _ as usize as *const T)
                as &T
        }
    }
    pub fn get_mut<T>(&mut self, offset: usize) -> &mut T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SZ);
        self.modified = true;
        unsafe {
            &mut *((&mut (self.cache[offset..offset + type_size])).as_mut_ptr() as *mut _ as usize
                as *mut T) as &mut T
        }
    }
    pub fn read<T, U>(&self, offset: usize, f: impl FnOnce(&T) -> U) -> U {
        f(self.get_ref(offset))
    }
    pub fn modify<T, U>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> U) -> U {
        f(self.get_mut(offset))
    }
    pub fn set_modify(&mut self) {
        self.modified = true;
    }
    pub fn is_modify(&mut self) -> bool {
        self.modified
    }
    pub fn sync(&mut self) {
        if self.modified {
            self.modified = false;
            self.block_dev
                .as_ref()
                .unwrap()
                .write_block(self.sector_id, &self.cache);
        }
    }
}

impl Drop for BlockCache {
    fn drop(&mut self) {
        self.sync()
    }
}

impl Default for BlockCache {
    fn default() -> Self {
        Self {
            cache: [0; BLOCK_SZ],
            sector_id: 0,
            block_dev: None, // Arc + dyn 实现 BlockDevice Trait 的动态分发
            modified: false,
        }
    }
}

pub type SectorCache = BlockCache;

pub struct SectorCacheManager {
    queue: VecDeque<(usize, Arc<Mutex<SectorCache>>)>,
}

impl SectorCacheManager {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
    pub fn get_sector_cache(
        &mut self,
        sector_id: usize,
        block_device: Arc<dyn BlockDevice>,
    ) -> Arc<Mutex<SectorCache>> {
        if let Some(pair) = self.queue.iter().find(|pair| pair.0 == sector_id) {
            Arc::clone(&pair.1)
        } else {
            // substitute
            if self.queue.len() == SECTOR_CACHE_SIZE {
                // from front to tail
                if let Some((idx, _)) = self
                    .queue
                    .iter()
                    .enumerate()
                    .find(|(_, pair)| Arc::strong_count(&pair.1) == 1)
                {
                    self.queue.drain(idx..=idx);
                } else {
                    panic!("Run out of SectorCache!");
                }
            }
            // load sector into mem and push back
            let sector_cache = Arc::new(Mutex::new(BlockCache::new(
                sector_id,
                Arc::clone(&block_device),
            )));
            self.queue.push_back((sector_id, Arc::clone(&sector_cache)));
            sector_cache
        }
    }
}

lazy_static! {
    pub static ref SECTOR_CACHE_MANAGER: SectorCacheManager = SectorCacheManager::new();
}

pub fn get_sector_cache(
    sector_id: usize,
    block_device: Arc<dyn BlockDevice>,
) -> Arc<Mutex<SectorCache>> {
    SECTOR_CACHE_MANAGER.get_sector_cache(sector_id, block_device)
}

pub fn sector_cache_sync_all() {
    let manager = SECTOR_CACHE_MANAGER;
    for (_, cache) in manager.queue.iter() {
        cache.lock().sync();
    }
}
