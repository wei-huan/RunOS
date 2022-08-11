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
    pub struct MapPermission: u8 {
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
    perm: MapPermission,
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
        perm: MapPermission,
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
    pub fn copy_data(&mut self, page_table: &mut PageTable, data: &[u8], offset: usize) {
        assert_eq!(self.map_type, MapType::Framed);
        let mut start: usize = 0;
        let mut page_offset: usize = offset;
        let mut current_vpn = self.vpn_range.get_start();
        let len = data.len();
        if len == 0 {
            return;
        }
        loop {
            let src = &data[start..len.min(start + PAGE_SIZE - page_offset)];
            let dst = &mut page_table
                .translate(current_vpn)
                .unwrap()
                .ppn()
                .get_bytes_array()[page_offset..(page_offset + src.len())];
            dst.copy_from_slice(src);
            start += PAGE_SIZE - page_offset;
            page_offset = 0;
            if start >= len {
                break;
            }
            current_vpn.step();
        }
    }
    /// data: start-aligned but maybe with shorter length
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
    pub fn get_section_range(&self) -> (VirtPageNum, VirtPageNum) {
        (self.vpn_range.get_start(), self.vpn_range.get_end())
    }
    pub fn modify_section_end(&mut self, page_table: &mut PageTable, new_end_vpn: VirtPageNum) {
        let end_vpn = self.vpn_range.get_end();
        // shrink
        if end_vpn > new_end_vpn {
            // println!(
            //     "shrink end_vpn: {:#?}, new_end_vpn: {:#?}",
            //     end_vpn, new_end_vpn
            // );
            for vpn in new_end_vpn..end_vpn {
                self.unmap_one_page(page_table, vpn);
            }
        }
        // expand
        else if end_vpn < new_end_vpn {
            // println!(
            //     "expand end_vpn: {:#?}, new_end_vpn: {:#?}",
            //     end_vpn, new_end_vpn
            // );
            for vpn in end_vpn..new_end_vpn {
                self.map_one_page(page_table, vpn);
            }
        }
    }
    pub fn modify_section_start(&mut self, page_table: &mut PageTable, new_start_vpn: VirtPageNum) {
        let start_vpn = self.vpn_range.get_start();
        // expand
        if start_vpn > new_start_vpn {
            for vpn in new_start_vpn..start_vpn {
                self.map_one_page(page_table, vpn);
            }
        }
        // shrink
        else if start_vpn < new_start_vpn {
            for vpn in start_vpn..new_start_vpn {
                self.unmap_one_page(page_table, vpn);
            }
        }
    }
    // new_start_vpn should be still map after modify
    pub fn divide_into_two(
        &mut self,
        page_table: &mut PageTable,
        left_part_end: VirtPageNum,
        right_part_start: VirtPageNum,
    ) -> (Section, Section) {
        let mut left_section = Section::from_another(self);
        let mut right_section = Section::from_another(self);
        left_section.modify_section_end(page_table, left_part_end);
        for left_vpn in left_section.vpn_range {
            let frame = self.data_frames.remove(&left_vpn).unwrap();
            left_section.data_frames.insert(left_vpn, frame);
        }
        right_section.modify_section_start(page_table, right_part_start);
        for right_vpn in right_section.vpn_range {
            let frame = self.data_frames.remove(&right_vpn).unwrap();
            left_section.data_frames.insert(right_vpn, frame);
        }
        (left_section, right_section)
    }
}
