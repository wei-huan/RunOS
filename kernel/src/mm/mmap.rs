
use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct MMapProts: usize {
        const PROT_NONE = 0;
        const PROT_READ = 1;
        const PROT_WRITE = 2;
        const PROT_EXEC = 4;
        const PROT_GROWSDOWN = 0x01000000;
        const PROT_GROWSUP = 0x02000000;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct MMapFlags: usize {
        const MAP_FILE = 0;
        const MAP_SHARED= 1;
        const MAP_PRIVATE = 1 << 1;
        const MAP_FIXED = 1 << 4;
        const MAP_ANONYMOUS = 1 << 5;
    }
}