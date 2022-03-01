use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};

const PA_WIDTH_SV39: usize = 56;
const VA_WIDTH_SV39: usize = 39;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysAddr(usize);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VirtAddr(usize);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysPageNum(usize);

impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << VA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PPN_WIDTH_SV39) - 1))
    }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(v: PhysAddr) -> Self {
        Self(v.0 >> PAGE_SIZE_BITS)
    }
}

impl From<PhysAddr> for usize {
    fn from(v: PhysAddr) -> Self {
        v.0
    }
}

impl From<VirtAddr> for usize {
    fn from(v: VirtAddr) -> Self {
        v.0
    }
}

impl From<PhysPageNum> for usize {
    fn from(v: PhysPageNum) -> Self {
        v.0
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(v: PhysPageNum) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

impl PhysAddr {
    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
    }
}

impl VirtAddr {

}

impl PhysPageNum {
    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysAddr = (*self).into();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut u8, PAGE_SIZE) }
    }
}

pub fn addr_test() {
    let addr: usize = 0x123456789abcdef;
    let pa = PhysAddr::from(addr);
    // println!("pa: 0x{:X}", pa.0);
    assert!((pa.0 & usize::MAX << PA_WIDTH_SV39) == 0 );
    let va = VirtAddr::from(addr);
    // println!("va: 0x{:X}", va.0);
    assert!((va.0 & usize::MAX << VA_WIDTH_SV39) == 0 );
    let ppn = PhysPageNum::from(pa);
    // println!("ppn: 0x{:X}", ppn.0);
    assert!((va.0 & usize::MAX << PPN_WIDTH_SV39) == 0 );
    println!("addr_test past!");
}
