use super::{
    address::{PhysPageNum, VirtPageNum},
    frame::{frame_alloc, Frame},
};
use alloc::vec;
use alloc::vec::Vec;
use bitflags::bitflags;

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

bitflags! {
    pub struct RSWField: u8 {
        const A = 1 << 0;
        const B = 1 << 1;
    }
}

#[repr(C)]
pub struct PageTableEntry(pub usize);

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, rsw: RSWField, flags: PTEFlags) -> Self {
        Self((ppn.0 << 10) | ((rsw.bits as usize) << 8) | flags.bits as usize)
    }

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn ppn(&self) -> PhysPageNum {
        PhysPageNum::from(self.0 >> 10)
    }

    pub fn rsw(&self) -> RSWField {
        RSWField::from_bits((self.0 >> 8) as u8 & 0x03).unwrap()
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.0 as u8).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }

    pub fn is_readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }

    pub fn is_writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }

    pub fn is_executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }

    pub fn is_user_access(&self) -> bool {
        (self.flags() & PTEFlags::U) != PTEFlags::empty()
    }
}

pub struct PageTable {
    root_ppn: PhysPageNum,
    pte_frames: Vec<Frame>,
}

impl PageTable {
    pub fn new() -> Self {
        let root_pagetable_frame = frame_alloc().unwrap();
        Self {
            root_ppn: root_pagetable_frame.ppn,
            pte_frames: vec![root_pagetable_frame],
        }
    }

    pub fn get_root_ppn(&self) -> PhysPageNum {
        self.root_ppn
    }

    pub fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for (i ,idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*idx];
            if !pte.is_valid() {
                return None;
            }
            if i == 2 {
                result = Some(pte);
                break;
            }
            ppn = pte.ppn();
        }
        result
    }

    pub fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for (i ,idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*idx];
            if !pte.is_valid() {
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, RSWField::empty(), PTEFlags::V);
                self.pte_frames.push(frame);
            }
            if i == 2 {
                result = Some(pte);
                break;
            }
            ppn = pte.ppn();
        }
        result
    }

    #[allow(unused)]
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let mut pte = self.find_pte_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
        *pte = PageTableEntry::new(ppn, RSWField::empty(), flags | PTEFlags::V);
    }

    #[allow(unused)]
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let mut pte = self.find_pte(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {:?} is already unmapped", vpn);
        *pte = PageTableEntry::empty();
    }
}
