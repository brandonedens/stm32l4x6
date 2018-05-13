#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - AConfiguration register 1"]
    pub acr1: ACR1,
    #[doc = "0x08 - AConfiguration register 2"]
    pub acr2: ACR2,
    #[doc = "0x0c - AFRCR"]
    pub afrcr: AFRCR,
    #[doc = "0x10 - ASlot register"]
    pub aslotr: ASLOTR,
    #[doc = "0x14 - AInterrupt mask register2"]
    pub aim: AIM,
    #[doc = "0x18 - AStatus register"]
    pub asr: ASR,
    #[doc = "0x1c - AClear flag register"]
    pub aclrfr: ACLRFR,
    #[doc = "0x20 - AData register"]
    pub adr: ADR,
    #[doc = "0x24 - BConfiguration register 1"]
    pub bcr1: BCR1,
    #[doc = "0x28 - BConfiguration register 2"]
    pub bcr2: BCR2,
    #[doc = "0x2c - BFRCR"]
    pub bfrcr: BFRCR,
    #[doc = "0x30 - BSlot register"]
    pub bslotr: BSLOTR,
    #[doc = "0x34 - BInterrupt mask register2"]
    pub bim: BIM,
    #[doc = "0x38 - BStatus register"]
    pub bsr: BSR,
    #[doc = "0x3c - BClear flag register"]
    pub bclrfr: BCLRFR,
    #[doc = "0x40 - BData register"]
    pub bdr: BDR,
}
#[doc = "BConfiguration register 1"]
pub struct BCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BConfiguration register 1"]
pub mod bcr1;
#[doc = "BConfiguration register 2"]
pub struct BCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BConfiguration register 2"]
pub mod bcr2;
#[doc = "BFRCR"]
pub struct BFRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BFRCR"]
pub mod bfrcr;
#[doc = "BSlot register"]
pub struct BSLOTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BSlot register"]
pub mod bslotr;
#[doc = "BInterrupt mask register2"]
pub struct BIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BInterrupt mask register2"]
pub mod bim;
#[doc = "BStatus register"]
pub struct BSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BStatus register"]
pub mod bsr;
#[doc = "BClear flag register"]
pub struct BCLRFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BClear flag register"]
pub mod bclrfr;
#[doc = "BData register"]
pub struct BDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BData register"]
pub mod bdr;
#[doc = "AConfiguration register 1"]
pub struct ACR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AConfiguration register 1"]
pub mod acr1;
#[doc = "AConfiguration register 2"]
pub struct ACR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AConfiguration register 2"]
pub mod acr2;
#[doc = "AFRCR"]
pub struct AFRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AFRCR"]
pub mod afrcr;
#[doc = "ASlot register"]
pub struct ASLOTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ASlot register"]
pub mod aslotr;
#[doc = "AInterrupt mask register2"]
pub struct AIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AInterrupt mask register2"]
pub mod aim;
#[doc = "AStatus register"]
pub struct ASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AStatus register"]
pub mod asr;
#[doc = "AClear flag register"]
pub struct ACLRFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AClear flag register"]
pub mod aclrfr;
#[doc = "AData register"]
pub struct ADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AData register"]
pub mod adr;
