pub struct PhysAddr(usize);

impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << 39) - 1))
    }
}

// impl PhysAddr{

// }

pub struct VirtAddr(usize);

impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << 56) - 1))
    }
}

// impl VirtAddr{

// }
