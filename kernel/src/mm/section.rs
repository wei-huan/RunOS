use super::{
    address::{PhysPageNum, VirtPageNum},
    address::{StepByOne, VPNRange, VirtAddr},
    frame::{frame_alloc, Frame},
    page_table::{PTEFlags, PageTable},
};
use crate::config::PAGE_SIZE;
use alloc::collections::BTreeMap;
use alloc::string::String;
use bitflags::bitflags;

bitflags! {
    pub struct Permission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MapType {
    Framed,
    Identical,
}

pub struct Section {
    pub name: String,
    perm: Permission,
    map_type: MapType,
    pub vpn_range: VPNRange,
    data_frames: BTreeMap<VirtPageNum, Frame>,
}

impl Section {
    pub fn new(
        name: String,
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_type: MapType,
        perm: Permission,
    ) -> Self {
        Self {
            name,
            perm,
            map_type,
            data_frames: BTreeMap::new(),
            vpn_range: VPNRange::new(start_va.floor(), end_va.ceil()),
        }
    }
    pub fn from_another(another: &Section) -> Self {
        Self {
            name: String::from(&another.name),
            vpn_range: VPNRange::new(another.vpn_range.get_start(), another.vpn_range.get_end()),
            data_frames: BTreeMap::new(),
            map_type: another.map_type,
            perm: another.perm,
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
            self.map_one_page(page_table, vpn);
        }
    }
    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap_one_page(page_table, vpn);
        }
    }
    /// data: start-aligned but maybe with shorter length
    /// assume that all frames were cleared before
    pub fn copy_data(&mut self, page_table: &mut PageTable, data: &[u8]) {
        assert_eq!(self.map_type, MapType::Framed);
        let mut start: usize = 0;
        let mut current_vpn = self.vpn_range.get_start();
        let len = data.len();
        if len == 0 {
            // println!(".bss");
            return;
        }
        loop {
            let src = &data[start..len.min(start + PAGE_SIZE)];
            let dst = &mut page_table
                .translate(current_vpn)
                .unwrap()
                .ppn()
                .get_bytes_array()[..src.len()];
            dst.copy_from_slice(src);
            start += PAGE_SIZE;
            if start >= len {
                break;
            }
            current_vpn.step();
        }
    }
    /// data: start-aligned but maybe with shorter length
    /// assume that all frames were cleared before
    pub fn clear(&mut self, page_table: &mut PageTable) {
        assert_eq!(self.map_type, MapType::Framed);
        let mut current_vpn = self.vpn_range.get_start();
        // println!("current_vpn: {:?}", current_vpn);
        let src = &[0; PAGE_SIZE];
        // println!("end_vpn: {:?}", self.vpn_range.get_end());
        if current_vpn == self.vpn_range.get_end() {
            return;
        }
        loop {
            // println!("clear");
            let dst = &mut page_table
                .translate(current_vpn)
                .unwrap()
                .ppn()
                .get_bytes_array()[..PAGE_SIZE];
            dst.copy_from_slice(src);
            current_vpn.step();
            if current_vpn == self.vpn_range.get_end() {
                break;
            }
        }
    }
}
