/// Base Extension

use crate::opensbi::opensbi_call;

const SBI_Base_EID: usize = 0x10;

const SBI_GET_SPEC_VERSION_FID: usize = 0;
const SBI_GET_IMPL_ID_FID: usize = 1;
const SBI_GET_IMPL_VERSION_FID: usize = 2;

/// SBI implementation name
#[derive(Debug, Clone, Copy)]
pub enum SbiImplId {
    #[allow(missing_docs)]
    BerkeleyBootLoader,
    #[allow(missing_docs)]
    OpenSbi,
    #[allow(missing_docs)]
    Xvisor,
    #[allow(missing_docs)]
    Kvm,
    #[allow(missing_docs)]
    RustSbi,
    #[allow(missing_docs)]
    Diosix,
    #[allow(missing_docs)]
    Other(usize),
}

impl From<usize> for SbiImplId {
    fn from(v: usize) -> Self {
        match v {
            0 => SbiImplId::BerkeleyBootLoader,
            1 => SbiImplId::OpenSbi,
            2 => SbiImplId::Xvisor,
            3 => SbiImplId::Kvm,
            4 => SbiImplId::RustSbi,
            5 => SbiImplId::Diosix,
            v => SbiImplId::Other(v),
        }
    }
}

impl From<SbiImplId> for usize {
    fn from(v: SbiImplId) -> usize {
        match v {
            SbiImplId::BerkeleyBootLoader => 0,
            SbiImplId::OpenSbi => 1,
            SbiImplId::Xvisor => 2,
            SbiImplId::Kvm => 3,
            SbiImplId::RustSbi => 4,
            SbiImplId::Diosix => 5,
            SbiImplId::Other(v) => v,
        }
    }
}

/// SBI specification version implemented by the SBI implementation
#[derive(Debug, Clone, Copy)]
pub struct SbiSpecVersion {
    /// Major version number
    pub major: usize,
    /// Minor version number
    pub minor: usize,
}

#[allow(unused)]
pub fn spec_version() -> SbiSpecVersion {
    let value = opensbi_call(SBI_Base_EID, SBI_GET_SPEC_VERSION_FID, 0, 0, 0, 0, 0, 0).1;
    SbiSpecVersion {
        major: (value >> 24) & 0x7f,
        minor: value & 0xff_ffff,
    }
}

#[allow(unused)]
pub fn impl_id() -> SbiImplId {
    let value = opensbi_call(SBI_Base_EID, SBI_GET_IMPL_ID_FID, 0, 0, 0, 0, 0, 0).1;
    value.into()
}

#[allow(unused)]
pub fn impl_version() -> usize {
    opensbi_call(SBI_Base_EID, SBI_GET_IMPL_VERSION_FID, 0, 0, 0, 0, 0, 0).1
}
