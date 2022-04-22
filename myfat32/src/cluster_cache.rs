/// 簇缓存层，扇区的进一步抽象，主要用于 FAT32 的数据区
use super::{BlockDevice, SectorCache, CLUS_SZ, DATA_START_SEC, SECS_PER_CLU, SEC_SZ};
use std::sync::Arc;
// use std::collections::VecDeque;
// use lazy_static::*;
// use spin::Mutex;

pub struct ClusterCache {
    cluster_cache: [SectorCache; SECS_PER_CLU],
    modified: bool,
    cluster_id: usize,                  // cluster_id 是数据区的簇号, 从 2 开始标号
}

impl ClusterCache {
    pub fn new(cluster_id: usize, block_device: Arc<dyn BlockDevice>) -> Self {
        let mut cluster_cache: [SectorCache; SECS_PER_CLU] = Default::default(); //[SectorCache::new_empty(block_device.clone()); SECS_PER_CLU];
        let block_id = (cluster_id - 2) * SECS_PER_CLU + DATA_START_SEC;
        for (i, id) in (block_id..(block_id + SECS_PER_CLU)).enumerate() {
            block_device.read_block(id, cluster_cache[i].get_cache_mut());
        }
        Self {
            cluster_id,
            cluster_cache,
            modified: false,
        }
    }
    pub fn get_ref<T>(&self, offset: usize) -> &T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= CLUS_SZ);
        let sec_id = offset / SEC_SZ;
        let sec_offset = offset % SEC_SZ;
        unsafe {
            &*((*(self.cluster_cache[sec_id].get_cache_ref()))[sec_offset..(sec_offset + type_size)]
                .as_ptr() as *const _ as usize as *const T) as &T
        }
    }
    pub fn get_mut<T>(&mut self, offset: usize) -> &mut T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= CLUS_SZ);
        self.modified = true;
        let sec_id = offset / SEC_SZ;
        let sec_offset = offset % SEC_SZ;
        self.cluster_cache[sec_id].set_modify();
        unsafe {
            &mut *((*(self.cluster_cache[sec_id].get_cache_mut()))
                [sec_offset..(sec_offset + type_size)]
                .as_mut_ptr() as *mut _ as usize as *mut T) as &mut T
        }
    }
    pub fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V {
        f(self.get_ref(offset))
    }
    pub fn modify<T, V>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> V) -> V {
        f(self.get_mut(offset))
    }
    pub fn sync(&mut self) {
        if self.modified {
            self.modified = false;
            // for i in SECS_PER_CLU.find(|(i, _)| self.cluster_cache[*i].is_modify() == true) {
            //     drop(self.cluster_cache[i])
            // }
            for i in 0..SECS_PER_CLU {
                if self.cluster_cache[i].is_modify() == true {
                    self.cluster_cache[i].sync()
                }
            }
        }
    }
}

impl Drop for ClusterCache {
    fn drop(&mut self) {
        self.sync()
    }
}
