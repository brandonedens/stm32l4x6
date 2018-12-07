#![doc = "Peripheral access API for STM32L4X6 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMP_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CHANNEL1();
    fn DMA1_CHANNEL2();
    fn DMA1_CHANNEL3();
    fn DMA1_CHANNEL4();
    fn DMA1_CHANNEL5();
    fn DMA1_CHANNEL6();
    fn DMA1_CHANNEL7();
    fn ADC1_2();
    fn CAN1_TX();
    fn CAN1_RX0();
    fn CAN1_RX1();
    fn CAN1_SCE();
    fn EXTI9_5();
    fn TIM15();
    fn TIM16();
    fn TIM1_TRG_COM_TIM17();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn DFSDM3();
    fn TIM8_BRK();
    fn TIM8();
    fn TIM8_TRG_COM();
    fn TIM8_CC();
    fn ADC3();
    fn FMC();
    fn SDMMC();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6_DAC();
    fn TIM7();
    fn DMA2_CHANNEL1();
    fn DMA2_CHANNEL2();
    fn DMA2_CHANNEL3();
    fn DMA2_CHANNEL4();
    fn DMA2_CHANNEL5();
    fn DFSDM0();
    fn DFSDM1();
    fn DFSDM2();
    fn COMP();
    fn LPTIM1();
    fn LPTIM2();
    fn OTG_FS();
    fn DMA2_CHANNEL6();
    fn DMA2_CHANNEL7();
    fn QUADSPI();
    fn I2C3_EV();
    fn I2C3_ER();
    fn SAI1();
    fn SAI2();
    fn SWPMI1();
    fn TSC();
    fn LCD();
    fn RNG();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 80] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector {
        _handler: TAMP_STAMP,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_CHANNEL1,
    },
    Vector {
        _handler: DMA1_CHANNEL2,
    },
    Vector {
        _handler: DMA1_CHANNEL3,
    },
    Vector {
        _handler: DMA1_CHANNEL4,
    },
    Vector {
        _handler: DMA1_CHANNEL5,
    },
    Vector {
        _handler: DMA1_CHANNEL6,
    },
    Vector {
        _handler: DMA1_CHANNEL7,
    },
    Vector { _handler: ADC1_2 },
    Vector { _handler: CAN1_TX },
    Vector { _handler: CAN1_RX0 },
    Vector { _handler: CAN1_RX1 },
    Vector { _handler: CAN1_SCE },
    Vector { _handler: EXTI9_5 },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector {
        _handler: TIM1_TRG_COM_TIM17,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector { _handler: DFSDM3 },
    Vector { _handler: TIM8_BRK },
    Vector { _handler: TIM8 },
    Vector {
        _handler: TIM8_TRG_COM,
    },
    Vector { _handler: TIM8_CC },
    Vector { _handler: ADC3 },
    Vector { _handler: FMC },
    Vector { _handler: SDMMC },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: TIM6_DAC },
    Vector { _handler: TIM7 },
    Vector {
        _handler: DMA2_CHANNEL1,
    },
    Vector {
        _handler: DMA2_CHANNEL2,
    },
    Vector {
        _handler: DMA2_CHANNEL3,
    },
    Vector {
        _handler: DMA2_CHANNEL4,
    },
    Vector {
        _handler: DMA2_CHANNEL5,
    },
    Vector { _handler: DFSDM0 },
    Vector { _handler: DFSDM1 },
    Vector { _handler: DFSDM2 },
    Vector { _handler: COMP },
    Vector { _handler: LPTIM1 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: OTG_FS },
    Vector {
        _handler: DMA2_CHANNEL6,
    },
    Vector {
        _handler: DMA2_CHANNEL7,
    },
    Vector { _reserved: 0 },
    Vector { _handler: QUADSPI },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector { _handler: SAI1 },
    Vector { _handler: SAI2 },
    Vector { _handler: SWPMI1 },
    Vector { _handler: TSC },
    Vector { _handler: LCD },
    Vector { _handler: RNG },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG,
    #[doc = "1 - PVD through EXTI line detection"]
    PVD,
    #[doc = "2 - Tamper and TimeStamp interrupts"]
    TAMP_STAMP,
    #[doc = "3 - RTC Tamper or TimeStamp /CSS on LSE through EXTI line 19 interrupts"]
    RTC_WKUP,
    #[doc = "4 - Flash global interrupt"]
    FLASH,
    #[doc = "5 - RCC global interrupt"]
    RCC,
    #[doc = "6 - EXTI Line 0 interrupt"]
    EXTI0,
    #[doc = "7 - EXTI Line 1 interrupt"]
    EXTI1,
    #[doc = "8 - EXTI Line 2 interrupt"]
    EXTI2,
    #[doc = "9 - EXTI Line 3 interrupt"]
    EXTI3,
    #[doc = "10 - EXTI Line4 interrupt"]
    EXTI4,
    #[doc = "11 - DMA1 Channel1 global interrupt"]
    DMA1_CHANNEL1,
    #[doc = "12 - DMA1 Channel2 global interrupt"]
    DMA1_CHANNEL2,
    #[doc = "13 - DMA1 Channel3 interrupt"]
    DMA1_CHANNEL3,
    #[doc = "14 - DMA1 Channel4 interrupt"]
    DMA1_CHANNEL4,
    #[doc = "15 - DMA1 Channel5 interrupt"]
    DMA1_CHANNEL5,
    #[doc = "16 - DMA1 Channel6 interrupt"]
    DMA1_CHANNEL6,
    #[doc = "17 - DMA1 Channel 7 interrupt"]
    DMA1_CHANNEL7,
    #[doc = "18 - ADC1 and ADC2 global interrupt"]
    ADC1_2,
    #[doc = "19 - CAN1 TX interrupts"]
    CAN1_TX,
    #[doc = "20 - CAN1 RX0 interrupts"]
    CAN1_RX0,
    #[doc = "21 - CAN1 RX1 interrupts"]
    CAN1_RX1,
    #[doc = "22 - CAN1 SCE interrupt"]
    CAN1_SCE,
    #[doc = "23 - EXTI Line5 to Line9 interrupts"]
    EXTI9_5,
    #[doc = "24 - Timer 15 global interrupt"]
    TIM15,
    #[doc = "25 - Timer 16 global interrupt"]
    TIM16,
    #[doc = "26 - TIM1 Trigger and Commutation interrupts and TIM17 global interrupt"]
    TIM1_TRG_COM_TIM17,
    #[doc = "27 - TIM1 Capture Compare interrupt"]
    TIM1_CC,
    #[doc = "28 - TIM2 global interrupt"]
    TIM2,
    #[doc = "29 - TIM3 global interrupt"]
    TIM3,
    #[doc = "30 - TIM4 global interrupt"]
    TIM4,
    #[doc = "31 - I2C1 event interrupt"]
    I2C1_EV,
    #[doc = "32 - I2C1 error interrupt"]
    I2C1_ER,
    #[doc = "33 - I2C2 event interrupt"]
    I2C2_EV,
    #[doc = "34 - I2C2 error interrupt"]
    I2C2_ER,
    #[doc = "35 - SPI1 global interrupt"]
    SPI1,
    #[doc = "36 - SPI2 global interrupt"]
    SPI2,
    #[doc = "37 - USART1 global interrupt"]
    USART1,
    #[doc = "38 - USART2 global interrupt"]
    USART2,
    #[doc = "39 - USART3 global interrupt"]
    USART3,
    #[doc = "40 - EXTI Lines 10 to 15 interrupts"]
    EXTI15_10,
    #[doc = "41 - RTC alarms through EXTI line 18 interrupts"]
    RTC_ALARM,
    #[doc = "42 - SD Filter 3 global Interrupt"]
    DFSDM3,
    #[doc = "43 - TIM8 Break Interrupt"]
    TIM8_BRK,
    #[doc = "44 - TIM8 Update Interrupt"]
    TIM8,
    #[doc = "45 - TIM8 Trigger and Commutation Interrupt"]
    TIM8_TRG_COM,
    #[doc = "46 - TIM8 Capture Compare Interrupt"]
    TIM8_CC,
    #[doc = "47 - ADC3 global Interrupt"]
    ADC3,
    #[doc = "48 - FMC global Interrupt"]
    FMC,
    #[doc = "49 - SDMMC global Interrupt"]
    SDMMC,
    #[doc = "50 - TIM5 global Interrupt"]
    TIM5,
    #[doc = "51 - SPI3 global Interrupt"]
    SPI3,
    #[doc = "52 - UART4 global Interrupt"]
    UART4,
    #[doc = "53 - UART5 global Interrupt"]
    UART5,
    #[doc = "54 - TIM6 global and DAC1 and 2 underrun error interrupts"]
    TIM6_DAC,
    #[doc = "55 - TIM7 global interrupt"]
    TIM7,
    #[doc = "56 - DMA2 Channel 1 global Interrupt"]
    DMA2_CHANNEL1,
    #[doc = "57 - DMA2 Channel 2 global Interrupt"]
    DMA2_CHANNEL2,
    #[doc = "58 - DMA2 Channel 3 global Interrupt"]
    DMA2_CHANNEL3,
    #[doc = "59 - DMA2 Channel 4 global Interrupt"]
    DMA2_CHANNEL4,
    #[doc = "60 - DMA2 Channel 5 global Interrupt"]
    DMA2_CHANNEL5,
    #[doc = "61 - SD Filter 0 global Interrupt"]
    DFSDM0,
    #[doc = "62 - SD Filter 1 global Interrupt"]
    DFSDM1,
    #[doc = "63 - SD Filter 2 global Interrupt"]
    DFSDM2,
    #[doc = "64 - COMP1 and COMP2 interrupts"]
    COMP,
    #[doc = "65 - LP TIM1 interrupt"]
    LPTIM1,
    #[doc = "66 - LP TIM2 interrupt"]
    LPTIM2,
    #[doc = "67 - USB OTG FS global Interrupt"]
    OTG_FS,
    #[doc = "68 - DMA2 Channel 6 global Interrupt"]
    DMA2_CHANNEL6,
    #[doc = "69 - DMA2 Channel 7 global Interrupt"]
    DMA2_CHANNEL7,
    #[doc = "71 - Quad SPI global interrupt"]
    QUADSPI,
    #[doc = "72 - I2C3 event interrupt"]
    I2C3_EV,
    #[doc = "73 - I2C3 error interrupt"]
    I2C3_ER,
    #[doc = "74 - SAI1 global interrupt"]
    SAI1,
    #[doc = "75 - SAI2 global interrupt"]
    SAI2,
    #[doc = "76 - SWPMI1 global interrupt"]
    SWPMI1,
    #[doc = "77 - TSC global interrupt"]
    TSC,
    #[doc = "78 - LCD global interrupt"]
    LCD,
    #[doc = "79 - RNG global interrupt"]
    RNG,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WWDG => 0,
            Interrupt::PVD => 1,
            Interrupt::TAMP_STAMP => 2,
            Interrupt::RTC_WKUP => 3,
            Interrupt::FLASH => 4,
            Interrupt::RCC => 5,
            Interrupt::EXTI0 => 6,
            Interrupt::EXTI1 => 7,
            Interrupt::EXTI2 => 8,
            Interrupt::EXTI3 => 9,
            Interrupt::EXTI4 => 10,
            Interrupt::DMA1_CHANNEL1 => 11,
            Interrupt::DMA1_CHANNEL2 => 12,
            Interrupt::DMA1_CHANNEL3 => 13,
            Interrupt::DMA1_CHANNEL4 => 14,
            Interrupt::DMA1_CHANNEL5 => 15,
            Interrupt::DMA1_CHANNEL6 => 16,
            Interrupt::DMA1_CHANNEL7 => 17,
            Interrupt::ADC1_2 => 18,
            Interrupt::CAN1_TX => 19,
            Interrupt::CAN1_RX0 => 20,
            Interrupt::CAN1_RX1 => 21,
            Interrupt::CAN1_SCE => 22,
            Interrupt::EXTI9_5 => 23,
            Interrupt::TIM15 => 24,
            Interrupt::TIM16 => 25,
            Interrupt::TIM1_TRG_COM_TIM17 => 26,
            Interrupt::TIM1_CC => 27,
            Interrupt::TIM2 => 28,
            Interrupt::TIM3 => 29,
            Interrupt::TIM4 => 30,
            Interrupt::I2C1_EV => 31,
            Interrupt::I2C1_ER => 32,
            Interrupt::I2C2_EV => 33,
            Interrupt::I2C2_ER => 34,
            Interrupt::SPI1 => 35,
            Interrupt::SPI2 => 36,
            Interrupt::USART1 => 37,
            Interrupt::USART2 => 38,
            Interrupt::USART3 => 39,
            Interrupt::EXTI15_10 => 40,
            Interrupt::RTC_ALARM => 41,
            Interrupt::DFSDM3 => 42,
            Interrupt::TIM8_BRK => 43,
            Interrupt::TIM8 => 44,
            Interrupt::TIM8_TRG_COM => 45,
            Interrupt::TIM8_CC => 46,
            Interrupt::ADC3 => 47,
            Interrupt::FMC => 48,
            Interrupt::SDMMC => 49,
            Interrupt::TIM5 => 50,
            Interrupt::SPI3 => 51,
            Interrupt::UART4 => 52,
            Interrupt::UART5 => 53,
            Interrupt::TIM6_DAC => 54,
            Interrupt::TIM7 => 55,
            Interrupt::DMA2_CHANNEL1 => 56,
            Interrupt::DMA2_CHANNEL2 => 57,
            Interrupt::DMA2_CHANNEL3 => 58,
            Interrupt::DMA2_CHANNEL4 => 59,
            Interrupt::DMA2_CHANNEL5 => 60,
            Interrupt::DFSDM0 => 61,
            Interrupt::DFSDM1 => 62,
            Interrupt::DFSDM2 => 63,
            Interrupt::COMP => 64,
            Interrupt::LPTIM1 => 65,
            Interrupt::LPTIM2 => 66,
            Interrupt::OTG_FS => 67,
            Interrupt::DMA2_CHANNEL6 => 68,
            Interrupt::DMA2_CHANNEL7 => 69,
            Interrupt::QUADSPI => 71,
            Interrupt::I2C3_EV => 72,
            Interrupt::I2C3_ER => 73,
            Interrupt::SAI1 => 74,
            Interrupt::SAI2 => 75,
            Interrupt::SWPMI1 => 76,
            Interrupt::TSC => 77,
            Interrupt::LCD => 78,
            Interrupt::RNG => 79,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Digital-to-analog converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac::RegisterBlock {
        1073771520 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &dac::RegisterBlock {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital-to-analog converter"]
pub mod dac;
#[doc = "Direct memory access controller"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma1::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    fn deref(&self) -> &dma1::RegisterBlock {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "Direct memory access controller"]
pub mod dma1;
#[doc = "DMA2"]
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma1::RegisterBlock {
        1073873920 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma1::RegisterBlock;
    fn deref(&self) -> &dma1::RegisterBlock {
        unsafe { &*DMA2::ptr() }
    }
}
#[doc = "Cyclic redundancy check calculation unit"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1073885184 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cyclic redundancy check calculation unit"]
pub mod crc;
#[doc = "Liquid crystal display controller"]
pub struct LCD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCD {}
impl LCD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lcd::RegisterBlock {
        1073751040 as *const _
    }
}
impl Deref for LCD {
    type Target = lcd::RegisterBlock;
    fn deref(&self) -> &lcd::RegisterBlock {
        unsafe { &*LCD::ptr() }
    }
}
#[doc = "Liquid crystal display controller"]
pub mod lcd;
#[doc = "Touch sensing controller"]
pub struct TSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSC {}
impl TSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tsc::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for TSC {
    type Target = tsc::RegisterBlock;
    fn deref(&self) -> &tsc::RegisterBlock {
        unsafe { &*TSC::ptr() }
    }
}
#[doc = "Touch sensing controller"]
pub mod tsc;
#[doc = "Independent watchdog"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iwdg::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    fn deref(&self) -> &iwdg::RegisterBlock {
        unsafe { &*IWDG::ptr() }
    }
}
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "System window watchdog"]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdg::RegisterBlock {
        1073753088 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    fn deref(&self) -> &wwdg::RegisterBlock {
        unsafe { &*WWDG::ptr() }
    }
}
#[doc = "System window watchdog"]
pub mod wwdg;
#[doc = "Comparator"]
pub struct COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP {}
impl COMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const comp::RegisterBlock {
        1073807872 as *const _
    }
}
impl Deref for COMP {
    type Target = comp::RegisterBlock;
    fn deref(&self) -> &comp::RegisterBlock {
        unsafe { &*COMP::ptr() }
    }
}
#[doc = "Comparator"]
pub mod comp;
#[doc = "Firewall"]
pub struct FIREWALL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FIREWALL {}
impl FIREWALL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const firewall::RegisterBlock {
        1073814528 as *const _
    }
}
impl Deref for FIREWALL {
    type Target = firewall::RegisterBlock;
    fn deref(&self) -> &firewall::RegisterBlock {
        unsafe { &*FIREWALL::ptr() }
    }
}
#[doc = "Firewall"]
pub mod firewall;
#[doc = "Inter-integrated circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073763328 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub mod i2c1;
#[doc = "I2C2"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073764352 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "I2C3"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073765376 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "Flash"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash::RegisterBlock {
        1073881088 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash"]
pub mod flash;
#[doc = "Debug support"]
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBGMCU {}
impl DBGMCU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dbgmcu::RegisterBlock {
        3758366720 as *const _
    }
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    fn deref(&self) -> &dbgmcu::RegisterBlock {
        unsafe { &*DBGMCU::ptr() }
    }
}
#[doc = "Debug support"]
pub mod dbgmcu;
#[doc = "QuadSPI interface"]
pub struct QUADSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QUADSPI {}
impl QUADSPI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const quadspi::RegisterBlock {
        2684358656 as *const _
    }
}
impl Deref for QUADSPI {
    type Target = quadspi::RegisterBlock;
    fn deref(&self) -> &quadspi::RegisterBlock {
        unsafe { &*QUADSPI::ptr() }
    }
}
#[doc = "QuadSPI interface"]
pub mod quadspi;
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcc::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    fn deref(&self) -> &rcc::RegisterBlock {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwr::RegisterBlock {
        1073770496 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    fn deref(&self) -> &pwr::RegisterBlock {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "Power control"]
pub mod pwr;
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const syscfg::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    fn deref(&self) -> &syscfg::RegisterBlock {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "Digital filter for sigma delta modulators"]
pub struct DFSDM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DFSDM {}
impl DFSDM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dfsdm::RegisterBlock {
        1073831936 as *const _
    }
}
impl Deref for DFSDM {
    type Target = dfsdm::RegisterBlock;
    fn deref(&self) -> &dfsdm::RegisterBlock {
        unsafe { &*DFSDM::ptr() }
    }
}
#[doc = "Digital filter for sigma delta modulators"]
pub mod dfsdm;
#[doc = "Random number generator"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rng::RegisterBlock {
        1342572544 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    fn deref(&self) -> &rng::RegisterBlock {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "Random number generator"]
pub mod rng;
#[doc = "Advanced encryption standard hardware accelerator"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aes::RegisterBlock {
        1342570496 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    fn deref(&self) -> &aes::RegisterBlock {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "Advanced encryption standard hardware accelerator"]
pub mod aes;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1342439424 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc1;
#[doc = "ADC2"]
pub struct ADC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC2 {}
impl ADC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1342439680 as *const _
    }
}
impl Deref for ADC2 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC2::ptr() }
    }
}
#[doc = "ADC3"]
pub struct ADC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC3 {}
impl ADC3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1342439936 as *const _
    }
}
impl Deref for ADC3 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC3::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub struct ADC123_COMMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC123_COMMON {}
impl ADC123_COMMON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc123_common::RegisterBlock {
        1342440192 as *const _
    }
}
impl Deref for ADC123_COMMON {
    type Target = adc123_common::RegisterBlock;
    fn deref(&self) -> &adc123_common::RegisterBlock {
        unsafe { &*ADC123_COMMON::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc123_common;
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1207959552 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiob::RegisterBlock {
        1207960576 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiob;
#[doc = "General-purpose I/Os"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1207961600 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioc;
#[doc = "GPIOD"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1207962624 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "GPIOE"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1207963648 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "GPIOF"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1207964672 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "GPIOG"]
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1207965696 as *const _
    }
}
impl Deref for GPIOG {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOG::ptr() }
    }
}
#[doc = "GPIOH"]
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1207966720 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOH::ptr() }
    }
}
#[doc = "Serial audio interface"]
pub struct SAI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI1 {}
impl SAI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sai1::RegisterBlock {
        1073828864 as *const _
    }
}
impl Deref for SAI1 {
    type Target = sai1::RegisterBlock;
    fn deref(&self) -> &sai1::RegisterBlock {
        unsafe { &*SAI1::ptr() }
    }
}
#[doc = "Serial audio interface"]
pub mod sai1;
#[doc = "SAI2"]
pub struct SAI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI2 {}
impl SAI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sai1::RegisterBlock {
        1073829888 as *const _
    }
}
impl Deref for SAI2 {
    type Target = sai1::RegisterBlock;
    fn deref(&self) -> &sai1::RegisterBlock {
        unsafe { &*SAI2::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM2::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim2;
#[doc = "TIM3"]
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073742848 as *const _
    }
}
impl Deref for TIM3 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM3::ptr() }
    }
}
#[doc = "TIM4"]
pub struct TIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM4 {}
impl TIM4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073743872 as *const _
    }
}
impl Deref for TIM4 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM4::ptr() }
    }
}
#[doc = "TIM5"]
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM5 {}
impl TIM5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073744896 as *const _
    }
}
impl Deref for TIM5 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM5::ptr() }
    }
}
#[doc = "General purpose timers"]
pub struct TIM15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM15 {}
impl TIM15 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim15::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for TIM15 {
    type Target = tim15::RegisterBlock;
    fn deref(&self) -> &tim15::RegisterBlock {
        unsafe { &*TIM15::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim15;
#[doc = "General purpose timers"]
pub struct TIM16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM16 {}
impl TIM16 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim16::RegisterBlock {
        1073824768 as *const _
    }
}
impl Deref for TIM16 {
    type Target = tim16::RegisterBlock;
    fn deref(&self) -> &tim16::RegisterBlock {
        unsafe { &*TIM16::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim16;
#[doc = "TIM17"]
pub struct TIM17 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM17 {}
impl TIM17 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim16::RegisterBlock {
        1073825792 as *const _
    }
}
impl Deref for TIM17 {
    type Target = tim16::RegisterBlock;
    fn deref(&self) -> &tim16::RegisterBlock {
        unsafe { &*TIM17::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim1::RegisterBlock {
        1073818624 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub mod tim1;
#[doc = "Advanced-timers"]
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM8 {}
impl TIM8 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim8::RegisterBlock {
        1073820672 as *const _
    }
}
impl Deref for TIM8 {
    type Target = tim8::RegisterBlock;
    fn deref(&self) -> &tim8::RegisterBlock {
        unsafe { &*TIM8::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub mod tim8;
#[doc = "Basic-timers"]
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM6 {}
impl TIM6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim6::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        unsafe { &*TIM6::ptr() }
    }
}
#[doc = "Basic-timers"]
pub mod tim6;
#[doc = "TIM7"]
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM7 {}
impl TIM7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim6::RegisterBlock {
        1073746944 as *const _
    }
}
impl Deref for TIM7 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        unsafe { &*TIM7::ptr() }
    }
}
#[doc = "Low power timer"]
pub struct LPTIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM1 {}
impl LPTIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lptim1::RegisterBlock {
        1073773568 as *const _
    }
}
impl Deref for LPTIM1 {
    type Target = lptim1::RegisterBlock;
    fn deref(&self) -> &lptim1::RegisterBlock {
        unsafe { &*LPTIM1::ptr() }
    }
}
#[doc = "Low power timer"]
pub mod lptim1;
#[doc = "LPTIM2"]
pub struct LPTIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM2 {}
impl LPTIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lptim1::RegisterBlock {
        1073779712 as *const _
    }
}
impl Deref for LPTIM2 {
    type Target = lptim1::RegisterBlock;
    fn deref(&self) -> &lptim1::RegisterBlock {
        unsafe { &*LPTIM2::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073821696 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
#[doc = "USART2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073759232 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "USART3"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073760256 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "UART4"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073761280 as *const _
    }
}
impl Deref for UART4 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "UART5"]
pub struct UART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART5 {}
impl UART5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073762304 as *const _
    }
}
impl Deref for UART5 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*UART5::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct LPUART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART1 {}
impl LPUART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpuart1::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for LPUART1 {
    type Target = lpuart1::RegisterBlock;
    fn deref(&self) -> &lpuart1::RegisterBlock {
        unsafe { &*LPUART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod lpuart1;
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub mod spi1;
#[doc = "SPI2"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073756160 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "SPI3"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073757184 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "Secure digital input/output interface"]
pub struct SDMMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC {}
impl SDMMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sdmmc::RegisterBlock {
        1073817600 as *const _
    }
}
impl Deref for SDMMC {
    type Target = sdmmc::RegisterBlock;
    fn deref(&self) -> &sdmmc::RegisterBlock {
        unsafe { &*SDMMC::ptr() }
    }
}
#[doc = "Secure digital input/output interface"]
pub mod sdmmc;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const exti::RegisterBlock {
        1073808384 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    fn deref(&self) -> &exti::RegisterBlock {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "Voltage reference buffer"]
pub struct VREF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREF {}
impl VREF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vref::RegisterBlock {
        1073807408 as *const _
    }
}
impl Deref for VREF {
    type Target = vref::RegisterBlock;
    fn deref(&self) -> &vref::RegisterBlock {
        unsafe { &*VREF::ptr() }
    }
}
#[doc = "Voltage reference buffer"]
pub mod vref;
#[doc = "Controller area network"]
pub struct CAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN {}
impl CAN {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can::RegisterBlock {
        1073767424 as *const _
    }
}
impl Deref for CAN {
    type Target = can::RegisterBlock;
    fn deref(&self) -> &can::RegisterBlock {
        unsafe { &*CAN::ptr() }
    }
}
#[doc = "Controller area network"]
pub mod can;
#[doc = "Real-time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073752064 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_GLOBAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_GLOBAL {}
impl OTG_FS_GLOBAL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_global::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for OTG_FS_GLOBAL {
    type Target = otg_fs_global::RegisterBlock;
    fn deref(&self) -> &otg_fs_global::RegisterBlock {
        unsafe { &*OTG_FS_GLOBAL::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_global;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_HOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_HOST {}
impl OTG_FS_HOST {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_host::RegisterBlock {
        1342178304 as *const _
    }
}
impl Deref for OTG_FS_HOST {
    type Target = otg_fs_host::RegisterBlock;
    fn deref(&self) -> &otg_fs_host::RegisterBlock {
        unsafe { &*OTG_FS_HOST::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_host;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_DEVICE {}
impl OTG_FS_DEVICE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_device::RegisterBlock {
        1342179328 as *const _
    }
}
impl Deref for OTG_FS_DEVICE {
    type Target = otg_fs_device::RegisterBlock;
    fn deref(&self) -> &otg_fs_device::RegisterBlock {
        unsafe { &*OTG_FS_DEVICE::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_device;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_PWRCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_PWRCLK {}
impl OTG_FS_PWRCLK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_pwrclk::RegisterBlock {
        1342180864 as *const _
    }
}
impl Deref for OTG_FS_PWRCLK {
    type Target = otg_fs_pwrclk::RegisterBlock;
    fn deref(&self) -> &otg_fs_pwrclk::RegisterBlock {
        unsafe { &*OTG_FS_PWRCLK::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_pwrclk;
#[doc = "Single Wire Protocol Master Interface"]
pub struct SWPMI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWPMI1 {}
impl SWPMI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swpmi1::RegisterBlock {
        1073776640 as *const _
    }
}
impl Deref for SWPMI1 {
    type Target = swpmi1::RegisterBlock;
    fn deref(&self) -> &swpmi1::RegisterBlock {
        unsafe { &*SWPMI1::ptr() }
    }
}
#[doc = "Single Wire Protocol Master Interface"]
pub mod swpmi1;
#[doc = "Operational amplifiers"]
pub struct OPAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OPAMP {}
impl OPAMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const opamp::RegisterBlock {
        1073772544 as *const _
    }
}
impl Deref for OPAMP {
    type Target = opamp::RegisterBlock;
    fn deref(&self) -> &opamp::RegisterBlock {
        unsafe { &*OPAMP::ptr() }
    }
}
#[doc = "Operational amplifiers"]
pub mod opamp;
#[doc = "Flexible memory controller"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fmc::RegisterBlock {
        1610612736 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    fn deref(&self) -> &fmc::RegisterBlock {
        unsafe { &*FMC::ptr() }
    }
}
#[doc = "Flexible memory controller"]
pub mod fmc;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "LCD"]
    pub LCD: LCD,
    #[doc = "TSC"]
    pub TSC: TSC,
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "WWDG"]
    pub WWDG: WWDG,
    #[doc = "COMP"]
    pub COMP: COMP,
    #[doc = "FIREWALL"]
    pub FIREWALL: FIREWALL,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "DBGMCU"]
    pub DBGMCU: DBGMCU,
    #[doc = "QUADSPI"]
    pub QUADSPI: QUADSPI,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "DFSDM"]
    pub DFSDM: DFSDM,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "ADC2"]
    pub ADC2: ADC2,
    #[doc = "ADC3"]
    pub ADC3: ADC3,
    #[doc = "ADC123_COMMON"]
    pub ADC123_COMMON: ADC123_COMMON,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "GPIOG"]
    pub GPIOG: GPIOG,
    #[doc = "GPIOH"]
    pub GPIOH: GPIOH,
    #[doc = "SAI1"]
    pub SAI1: SAI1,
    #[doc = "SAI2"]
    pub SAI2: SAI2,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "TIM4"]
    pub TIM4: TIM4,
    #[doc = "TIM5"]
    pub TIM5: TIM5,
    #[doc = "TIM15"]
    pub TIM15: TIM15,
    #[doc = "TIM16"]
    pub TIM16: TIM16,
    #[doc = "TIM17"]
    pub TIM17: TIM17,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "TIM8"]
    pub TIM8: TIM8,
    #[doc = "TIM6"]
    pub TIM6: TIM6,
    #[doc = "TIM7"]
    pub TIM7: TIM7,
    #[doc = "LPTIM1"]
    pub LPTIM1: LPTIM1,
    #[doc = "LPTIM2"]
    pub LPTIM2: LPTIM2,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "UART5"]
    pub UART5: UART5,
    #[doc = "LPUART1"]
    pub LPUART1: LPUART1,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "SDMMC"]
    pub SDMMC: SDMMC,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "VREF"]
    pub VREF: VREF,
    #[doc = "CAN"]
    pub CAN: CAN,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "OTG_FS_GLOBAL"]
    pub OTG_FS_GLOBAL: OTG_FS_GLOBAL,
    #[doc = "OTG_FS_HOST"]
    pub OTG_FS_HOST: OTG_FS_HOST,
    #[doc = "OTG_FS_DEVICE"]
    pub OTG_FS_DEVICE: OTG_FS_DEVICE,
    #[doc = "OTG_FS_PWRCLK"]
    pub OTG_FS_PWRCLK: OTG_FS_PWRCLK,
    #[doc = "SWPMI1"]
    pub SWPMI1: SWPMI1,
    #[doc = "OPAMP"]
    pub OPAMP: OPAMP,
    #[doc = "FMC"]
    pub FMC: FMC,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            DAC: DAC {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            LCD: LCD {
                _marker: PhantomData,
            },
            TSC: TSC {
                _marker: PhantomData,
            },
            IWDG: IWDG {
                _marker: PhantomData,
            },
            WWDG: WWDG {
                _marker: PhantomData,
            },
            COMP: COMP {
                _marker: PhantomData,
            },
            FIREWALL: FIREWALL {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            DBGMCU: DBGMCU {
                _marker: PhantomData,
            },
            QUADSPI: QUADSPI {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            DFSDM: DFSDM {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            ADC2: ADC2 {
                _marker: PhantomData,
            },
            ADC3: ADC3 {
                _marker: PhantomData,
            },
            ADC123_COMMON: ADC123_COMMON {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            GPIOG: GPIOG {
                _marker: PhantomData,
            },
            GPIOH: GPIOH {
                _marker: PhantomData,
            },
            SAI1: SAI1 {
                _marker: PhantomData,
            },
            SAI2: SAI2 {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            TIM4: TIM4 {
                _marker: PhantomData,
            },
            TIM5: TIM5 {
                _marker: PhantomData,
            },
            TIM15: TIM15 {
                _marker: PhantomData,
            },
            TIM16: TIM16 {
                _marker: PhantomData,
            },
            TIM17: TIM17 {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            TIM8: TIM8 {
                _marker: PhantomData,
            },
            TIM6: TIM6 {
                _marker: PhantomData,
            },
            TIM7: TIM7 {
                _marker: PhantomData,
            },
            LPTIM1: LPTIM1 {
                _marker: PhantomData,
            },
            LPTIM2: LPTIM2 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            UART5: UART5 {
                _marker: PhantomData,
            },
            LPUART1: LPUART1 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            SDMMC: SDMMC {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            VREF: VREF {
                _marker: PhantomData,
            },
            CAN: CAN {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            OTG_FS_GLOBAL: OTG_FS_GLOBAL {
                _marker: PhantomData,
            },
            OTG_FS_HOST: OTG_FS_HOST {
                _marker: PhantomData,
            },
            OTG_FS_DEVICE: OTG_FS_DEVICE {
                _marker: PhantomData,
            },
            OTG_FS_PWRCLK: OTG_FS_PWRCLK {
                _marker: PhantomData,
            },
            SWPMI1: SWPMI1 {
                _marker: PhantomData,
            },
            OPAMP: OPAMP {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
        }
    }
}
