use super::{
    address::VirtAddr,
    address::{PhysPageNum, VirtPageNum},
    frame::{frame_alloc, Frame},
    page_table::{PTEFlags, PageTable},
};
use alloc::collections::BTreeMap;
use bitflags::bitflags;

bitflags! {
    pub struct Permission: u8 {
        const R = 1 << 0;
        const W = 1 << 1;
        const X = 1 << 2;
        const U = 1 << 3;
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MapType {
    Identical,
    Framed,
}

pub struct Section {
    perm: Permission,
    map_type: MapType,
    vpn_range: VPNRange,
    data_frames: BTreeMap<VirtPageNum, Frame>,
}

impl Section {
    pub fn new(start_va: VirtAddr, end_va: VirtAddr, map_type: MapType, perm: Permission) -> Self {
        Self {
            perm: perm,
            data_frames: BTreeMap::new(),
            map_type,
        }
    }
    pub fn map_one_page(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Identical => {
                ppn = PhysPageNum(vpn.0);
            }
            MapType::Framed => {
                let frame = frame_alloc().unwrap();
                ppn = frame.ppn;
                self.data_frames.insert(vpn, frame);
            }
        }
        let pte_flags = PTEFlags::from_bits(self.perm.bits).unwrap();
        page_table.map(vpn, ppn, pte_flags);
    }

    pub fn unmap_one_page(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        if self.map_type == MapType::Framed {
            self.data_frames.remove(&vpn);
        }
        page_table.unmap(vpn);
    }
    pub fn map(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap_one_page(page_table, vpn);
        }
    }
    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap_one_page(page_table, vpn);
        }
    }
}
