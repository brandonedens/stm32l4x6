// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Author: Jorge Aparicio <japaricious@gmail.com>
// Author: Brandon Edens <brandonedens@gmail.com>
// Date: 2016-11-19

//! Memory map for STM32L4x6 microcontrollers

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate volatile_register;

#[allow(missing_docs)]
pub mod dac;
#[allow(missing_docs)]
pub mod dma;
#[allow(missing_docs)]
pub mod crc;
#[allow(missing_docs)]
pub mod lcd;
#[allow(missing_docs)]
pub mod tsc;
#[allow(missing_docs)]
pub mod iwdg;
#[allow(missing_docs)]
pub mod wwdg;
#[allow(missing_docs)]
pub mod comp;
#[allow(missing_docs)]
pub mod firewall;
#[allow(missing_docs)]
pub mod i2c;
#[allow(missing_docs)]
pub mod flash;
#[allow(missing_docs)]
pub mod dbgmcu;
#[allow(missing_docs)]
pub mod quadspi;
#[allow(missing_docs)]
pub mod rcc;
#[allow(missing_docs)]
pub mod pwr;
#[allow(missing_docs)]
pub mod syscfg;
#[allow(missing_docs)]
pub mod dfsdm;
#[allow(missing_docs)]
pub mod rng;
#[allow(missing_docs)]
pub mod aes;
#[allow(missing_docs)]
pub mod adc;
#[allow(missing_docs)]
pub mod adc_common;
#[allow(missing_docs)]
pub mod gpio;
#[allow(missing_docs)]
pub mod sai;
#[allow(missing_docs)]
pub mod basic_timer;
#[allow(missing_docs)]
pub mod general_timer;
#[allow(missing_docs)]
pub mod general_2channel_timer;
#[allow(missing_docs)]
pub mod general_1channel_timer;
#[allow(missing_docs)]
pub mod advanced_timer;
#[allow(missing_docs)]
pub mod low_power_timer;
#[allow(missing_docs)]
pub mod usart;
#[allow(missing_docs)]
pub mod lpuart;
#[allow(missing_docs)]
pub mod spi;
#[allow(missing_docs)]
pub mod sdmmc;
#[allow(missing_docs)]
pub mod exti;
#[allow(missing_docs)]
pub mod vref;
#[allow(missing_docs)]
pub mod can;
#[allow(missing_docs)]
pub mod rtc;
#[allow(missing_docs)]
pub mod otg_fs_global;
#[allow(missing_docs)]
pub mod otg_fs_host;
#[allow(missing_docs)]
pub mod otg_fs_device;
#[allow(missing_docs)]
pub mod otg_fs_pwrclk;
#[allow(missing_docs)]
pub mod swpmi;
#[allow(missing_docs)]
pub mod opamp;
#[allow(missing_docs)]
pub mod fmc;
#[allow(missing_docs)]
pub mod nvic;

use dac::Dac;
use dma::Dma;
use crc::Crc;
use lcd::Lcd;
use tsc::Tsc;
use iwdg::Iwdg;
use wwdg::Wwdg;
use comp::Comp;
use firewall::Firewall;
use i2c::I2c;
use flash::Flash;
use dbgmcu::Dbgmcu;
use quadspi::Quadspi;
use rcc::Rcc;
use pwr::Pwr;
use syscfg::Syscfg;
use dfsdm::Dfsdm;
use rng::Rng;
use aes::Aes;
use adc::Adc;
use adc_common::AdcCommon;
use gpio::Gpio;
use sai::Sai;
use basic_timer::BasicTimer;
use general_timer::GeneralTimer;
use general_2channel_timer::General2ChannelTimer;
use general_1channel_timer::General1ChannelTimer;
use advanced_timer::AdvancedTimer;
use low_power_timer::LowPowerTimer;
use usart::Usart;
use lpuart::Lpuart;
use spi::Spi;
use sdmmc::Sdmmc;
use exti::Exti;
use vref::Vref;
use can::Can;
use rtc::Rtc;
use otg_fs_global::OtgFsGlobal;
use otg_fs_host::OtgFsHost;
use otg_fs_device::OtgFsDevice;
use otg_fs_pwrclk::OtgFsPwrclk;
use swpmi::Swpmi;
use opamp::Opamp;
use fmc::Fmc;
use nvic::Nvic;

const DAC: usize = 0x40007400;
const DMA1: usize = 0x40020000;
const DMA2: usize = 0x40020400;
const CRC: usize = 0x40023000;
const LCD: usize = 0x40002400;
const TSC: usize = 0x40024000;
const IWDG: usize = 0x40003000;
const WWDG: usize = 0x40002c00;
const COMP: usize = 0x40010200;
const FIREWALL: usize = 0x40011c00;
const I2C1: usize = 0x40005400;
const I2C2: usize = 0x40005800;
const I2C3: usize = 0x40005c00;
const FLASH: usize = 0x40022000;
const DBGMCU: usize = 0xe0042000;
const QUADSPI: usize = 0xa0001000;
const RCC: usize = 0x40021000;
const PWR: usize = 0x40007000;
const SYSCFG: usize = 0x40010000;
const DFSDM: usize = 0x40016000;
const RNG: usize = 0x50060800;
const AES: usize = 0x50060000;
const ADC1: usize = 0x50040000;
const ADC2: usize = 0x50040100;
const ADC3: usize = 0x50040200;
const ADC123_COMMON: usize = 0x50040300;
const GPIOA: usize = 0x48000000;
const GPIOB: usize = 0x48000400;
const GPIOC: usize = 0x48000800;
const GPIOD: usize = 0x48000c00;
const GPIOE: usize = 0x48001000;
const GPIOF: usize = 0x48001400;
const GPIOG: usize = 0x48001800;
const GPIOH: usize = 0x48001c00;
const SAI1: usize = 0x40015400;
const SAI2: usize = 0x40015800;
const TIM2: usize = 0x40000000;
const TIM3: usize = 0x40000400;
const TIM4: usize = 0x40000800;
const TIM5: usize = 0x40000c00;
const TIM15: usize = 0x40014000;
const TIM16: usize = 0x40014400;
const TIM17: usize = 0x40014800;
const TIM1: usize = 0x40012c00;
const TIM8: usize = 0x40013400;
const TIM6: usize = 0x40001000;
const TIM7: usize = 0x40001400;
const LPTIM1: usize = 0x40007c00;
const LPTIM2: usize = 0x40009400;
const USART1: usize = 0x40013800;
const USART2: usize = 0x40004400;
const USART3: usize = 0x40004800;
const UART4: usize = 0x40004c00;
const UART5: usize = 0x40005000;
const LPUART1: usize = 0x40008000;
const SPI1: usize = 0x40013000;
const SPI2: usize = 0x40003800;
const SPI3: usize = 0x40003c00;
const SDMMC: usize = 0x40012800;
const EXTI: usize = 0x40010400;
const VREF: usize = 0x40010030;
const CAN: usize = 0x40006400;
const RTC: usize = 0x40002800;
const OTG_FS_GLOBAL: usize = 0x50000000;
const OTG_FS_HOST: usize = 0x50000400;
const OTG_FS_DEVICE: usize = 0x50000800;
const OTG_FS_PWRCLK: usize = 0x50000e00;
const SWPMI1: usize = 0x40008800;
const OPAMP: usize = 0x40007800;
const FMC: usize = 0x60000000;
const NVIC: usize = 0xe000e000;

unsafe fn deref<T>(address: usize) -> &'static T {
    &*(address as *const T)
}

unsafe fn deref_mut<T>(address: usize) -> &'static mut T {
    &mut *(address as *mut T)
}

/// DAC register block (&'static)
pub fn dac() -> &'static Dac {
    unsafe { deref(DAC) }
}

/// DAC register block (&'static mut)
pub unsafe fn dac_mut() -> &'static mut Dac {
    deref_mut(DAC)
}

/// DMA1 register block (&'static)
pub fn dma1() -> &'static Dma {
    unsafe { deref(DMA1) }
}

/// DMA1 register block (&'static_mut)
pub unsafe fn dma1_mut() -> &'static mut Dma {
    deref_mut(DMA1)
}

/// DMA2 register block (&'static)
pub fn dma2() -> &'static Dma {
    unsafe { deref(DMA2) }
}

/// DMA2 register block (&'static_mut)
pub unsafe fn dma2_mut() -> &'static mut Dma {
    deref_mut(DMA2)
}

/// CRC register block (&'static)
pub fn crc() -> &'static Crc {
    unsafe { deref(CRC) }
}

/// CRC register block (&'static_mut)
pub unsafe fn crc_mut() -> &'static mut Crc {
    deref_mut(CRC)
}

/// LCD register block (&'static)
pub fn lcd() -> &'static Lcd {
    unsafe { deref(LCD) }
}

/// LCD register block (&'static_mut)
pub unsafe fn lcd_mut() -> &'static mut Lcd {
    deref_mut(LCD)
}

/// TSC register block (&'static)
pub fn tsc() -> &'static Tsc {
    unsafe { deref(TSC) }
}

/// TSC register block (&'static_mut)
pub unsafe fn tsc_mut() -> &'static mut Tsc {
    deref_mut(TSC)
}

/// IWDG register block (&'static)
pub fn iwdg() -> &'static Iwdg {
    unsafe { deref(IWDG) }
}

/// IWDG register block (&'static_mut)
pub unsafe fn iwdg_mut() -> &'static mut Iwdg {
    deref_mut(IWDG)
}

/// WWDG register block (&'static)
pub fn wwdg() -> &'static Wwdg {
    unsafe { deref(WWDG) }
}

/// WWDG register block (&'static_mut)
pub unsafe fn wwdg_mut() -> &'static mut Wwdg {
    deref_mut(WWDG)
}

/// COMP register block (&'static)
pub fn comp() -> &'static Comp {
    unsafe { deref(COMP) }
}

/// COMP register block (&'static_mut)
pub unsafe fn comp_mut() -> &'static mut Comp {
    deref_mut(COMP)
}

/// Firewall register block (&'static)
pub fn firewall() -> &'static Firewall {
    unsafe { deref(FIREWALL) }
}

/// Firewall register block (&'static_mut)
pub unsafe fn firewall_mut() -> &'static mut Firewall {
    deref_mut(FIREWALL)
}

/// I2C1 register block (&'static)
pub fn i2c1() -> &'static I2c {
    unsafe { deref(I2C1) }
}

/// I2C1 register block (&'static_mut)
pub unsafe fn i2c1_mut() -> &'static mut I2c {
    deref_mut(I2C1)
}

/// I2C2 register block (&'static)
pub fn i2c2() -> &'static I2c {
    unsafe { deref(I2C2) }
}

/// I2C2 register block (&'static_mut)
pub unsafe fn i2c2_mut() -> &'static mut I2c {
    deref_mut(I2C2)
}

/// I2C3 register block (&'static)
pub fn i2c3() -> &'static I2c {
    unsafe { deref(I2C3) }
}

/// I2C3 register block (&'static_mut)
pub unsafe fn i2c3_mut() -> &'static mut I2c {
    deref_mut(I2C3)
}

/// Flash register block (&'static)
pub fn flash() -> &'static Flash {
    unsafe { deref(FLASH) }
}

/// Flash register block (&'static_mut)
pub unsafe fn flash_mut() -> &'static mut Flash {
    deref_mut(FLASH)
}

/// DBGMCU register block (&'static)
pub fn dbgmcu() -> &'static Dbgmcu {
    unsafe { deref(DBGMCU) }
}

/// DBGMCU register block (&'static_mut)
pub unsafe fn dbgmcu_mut() -> &'static mut Dbgmcu {
    deref_mut(DBGMCU)
}

/// QUADSPI register block (&'static)
pub fn quadspi() -> &'static Quadspi {
    unsafe { deref(QUADSPI) }
}

/// QUADSPI register block (&'static_mut)
pub unsafe fn quadspi_mut() -> &'static mut Quadspi {
    deref_mut(QUADSPI)
}

/// RCC register block (&'static)
pub fn rcc() -> &'static Rcc {
    unsafe { deref(RCC) }
}

/// RCC register block (&'static_mut)
pub unsafe fn rcc_mut() -> &'static mut Rcc {
    deref_mut(RCC)
}

/// PWR register block (&'static)
pub fn pwr() -> &'static Pwr {
    unsafe { deref(PWR) }
}

/// PWR register block (&'static_mut)
pub unsafe fn pwr_mut() -> &'static mut Pwr {
    deref_mut(PWR)
}

/// SYSCFG register block (&'static)
pub fn syscfg() -> &'static Syscfg {
    unsafe { deref(SYSCFG) }
}

/// SYSCFG register block (&'static_mut)
pub unsafe fn syscfg_mut() -> &'static mut Syscfg {
    deref_mut(SYSCFG)
}

/// DFSDM register block (&'static)
pub fn dfsdm() -> &'static Dfsdm {
    unsafe { deref(DFSDM) }
}

/// DFSDM register block (&'static_mut)
pub unsafe fn dfsdm_mut() -> &'static mut Dfsdm {
    deref_mut(DFSDM)
}

/// RNG register block (&'static)
pub fn rng() -> &'static Rng {
    unsafe { deref(RNG) }
}

/// RNG register block (&'static_mut)
pub unsafe fn rng_mut() -> &'static mut Rng {
    deref_mut(RNG)
}

/// AES register block (&'static)
pub fn aes() -> &'static Aes {
    unsafe { deref(AES) }
}

/// AES register block (&'static_mut)
pub unsafe fn aes_mut() -> &'static mut Aes {
    deref_mut(AES)
}

/// ADC1 register block (&'static)
pub fn adc1() -> &'static Adc {
    unsafe { deref(ADC1) }
}

/// ADC1 register block (&'static_mut)
pub unsafe fn adc1_mut() -> &'static mut Adc {
    deref_mut(ADC1)
}

/// ADC2 register block (&'static)
pub fn adc2() -> &'static Adc {
    unsafe { deref(ADC2) }
}

/// ADC2 register block (&'static_mut)
pub unsafe fn adc2_mut() -> &'static mut Adc {
    deref_mut(ADC2)
}

/// ADC3 register block (&'static)
pub fn adc3() -> &'static Adc {
    unsafe { deref(ADC3) }
}

/// ADC3 register block (&'static_mut)
pub unsafe fn adc3_mut() -> &'static mut Adc {
    deref_mut(ADC3)
}

/// ADC123_Common register block (&'static)
pub fn adc_common() -> &'static AdcCommon {
    unsafe { deref(ADC123_COMMON) }
}

/// ADC123_Common register block (&'static_mut)
pub unsafe fn adc_common_mut() -> &'static mut AdcCommon {
    deref_mut(ADC123_COMMON)
}

/// GPIOA register block (&'static)
pub fn gpioa() -> &'static Gpio {
    unsafe { deref(GPIOA) }
}

/// GPIOA register block (&'static_mut)
pub unsafe fn gpioa_mut() -> &'static mut Gpio {
    deref_mut(GPIOA)
}

/// GPIOB register block (&'static)
pub fn gpiob() -> &'static Gpio {
    unsafe { deref(GPIOB) }
}

/// GPIOB register block (&'static_mut)
pub unsafe fn gpiob_mut() -> &'static mut Gpio {
    deref_mut(GPIOB)
}

/// GPIOC register block (&'static)
pub fn gpioc() -> &'static Gpio {
    unsafe { deref(GPIOC) }
}

/// GPIOC register block (&'static_mut)
pub unsafe fn gpioc_mut() -> &'static mut Gpio {
    deref_mut(GPIOC)
}

/// GPIOD register block (&'static)
pub fn gpiod() -> &'static Gpio {
    unsafe { deref(GPIOD) }
}

/// GPIOD register block (&'static_mut)
pub unsafe fn gpiod_mut() -> &'static mut Gpio {
    deref_mut(GPIOD)
}

/// GPIOE register block (&'static)
pub fn gpioe() -> &'static Gpio {
    unsafe { deref(GPIOE) }
}

/// GPIOE register block (&'static_mut)
pub unsafe fn gpioe_mut() -> &'static mut Gpio {
    deref_mut(GPIOE)
}

/// GPIOF register block (&'static)
pub fn gpiof() -> &'static Gpio {
    unsafe { deref(GPIOF) }
}

/// GPIOF register block (&'static_mut)
pub unsafe fn gpiof_mut() -> &'static mut Gpio {
    deref_mut(GPIOF)
}

/// GPIOG register block (&'static)
pub fn gpiog() -> &'static Gpio {
    unsafe { deref(GPIOG) }
}

/// GPIOG register block (&'static_mut)
pub unsafe fn gpiog_mut() -> &'static mut Gpio {
    deref_mut(GPIOG)
}

/// GPIOH register block (&'static)
pub fn gpioh() -> &'static Gpio {
    unsafe { deref(GPIOH) }
}

/// GPIOH register block (&'static_mut)
pub unsafe fn gpioh_mut() -> &'static mut Gpio {
    deref_mut(GPIOH)
}

/// SAI1 register block (&'static)
pub fn sai1() -> &'static Sai {
    unsafe { deref(SAI1) }
}

/// SAI1 register block (&'static_mut)
pub unsafe fn sai1_mut() -> &'static mut Sai {
    deref_mut(SAI1)
}

/// SAI2 register block (&'static)
pub fn sai2() -> &'static Sai {
    unsafe { deref(SAI2) }
}

/// SAI2 register block (&'static_mut)
pub unsafe fn sai2_mut() -> &'static mut Sai {
    deref_mut(SAI2)
}

/// TIM2 register block (&'static)
pub fn tim2() -> &'static GeneralTimer {
    unsafe { deref(TIM2) }
}

/// TIM2 register block (&'static_mut)
pub unsafe fn tim2_mut() -> &'static mut GeneralTimer {
    deref_mut(TIM2)
}

/// TIM3 register block (&'static)
pub fn tim3() -> &'static GeneralTimer {
    unsafe { deref(TIM3) }
}

/// TIM3 register block (&'static_mut)
pub unsafe fn tim3_mut() -> &'static mut GeneralTimer {
    deref_mut(TIM3)
}

/// TIM4 register block (&'static)
pub fn tim4() -> &'static GeneralTimer {
    unsafe { deref(TIM4) }
}

/// TIM4 register block (&'static_mut)
pub unsafe fn tim4_mut() -> &'static mut GeneralTimer {
    deref_mut(TIM4)
}

/// TIM5 register block (&'static)
pub fn tim5() -> &'static GeneralTimer {
    unsafe { deref(TIM5) }
}

/// TIM5 register block (&'static_mut)
pub unsafe fn tim5_mut() -> &'static mut GeneralTimer {
    deref_mut(TIM5)
}

/// TIM15 register block (&'static)
pub fn tim15() -> &'static General2ChannelTimer {
    unsafe { deref(TIM15) }
}

/// TIM15 register block (&'static_mut)
pub unsafe fn tim15_mut() -> &'static mut General2ChannelTimer {
    deref_mut(TIM15)
}

/// TIM16 register block (&'static)
pub fn tim16() -> &'static General1ChannelTimer {
    unsafe { deref(TIM16) }
}

/// TIM16 register block (&'static_mut)
pub unsafe fn tim16_mut() -> &'static mut General1ChannelTimer {
    deref_mut(TIM16)
}

/// TIM17 register block (&'static)
pub fn tim17() -> &'static General1ChannelTimer {
    unsafe { deref(TIM17) }
}

/// TIM17 register block (&'static_mut)
pub unsafe fn tim17_mut() -> &'static mut General1ChannelTimer {
    deref_mut(TIM17)
}

/// TIM1 register block (&'static)
pub fn tim1() -> &'static AdvancedTimer {
    unsafe { deref(TIM1) }
}

/// TIM1 register block (&'static_mut)
pub unsafe fn tim1_mut() -> &'static mut AdvancedTimer {
    deref_mut(TIM1)
}

/// TIM8 register block (&'static)
pub fn tim8() -> &'static AdvancedTimer {
    unsafe { deref(TIM8) }
}

/// TIM8 register block (&'static_mut)
pub unsafe fn tim8_mut() -> &'static mut AdvancedTimer {
    deref_mut(TIM8)
}

/// TIM6 register block (&'static)
pub fn tim6() -> &'static BasicTimer {
    unsafe { deref(TIM6) }
}

/// TIM6 register block (&'static_mut)
pub unsafe fn tim6_mut() -> &'static mut BasicTimer {
    deref_mut(TIM6)
}

/// TIM7 register block (&'static)
pub fn tim7() -> &'static BasicTimer {
    unsafe { deref(TIM7) }
}

/// TIM7 register block (&'static_mut)
pub unsafe fn tim7_mut() -> &'static mut BasicTimer {
    deref_mut(TIM7)
}

/// LPTIM1 register block (&'static)
pub fn lptim1() -> &'static LowPowerTimer {
    unsafe { deref(LPTIM1) }
}

/// LPTIM1 register block (&'static_mut)
pub unsafe fn lptim1_mut() -> &'static mut LowPowerTimer {
    deref_mut(LPTIM1)
}

/// LPTIM2 register block (&'static)
pub fn lptim2() -> &'static LowPowerTimer {
    unsafe { deref(LPTIM2) }
}

/// LPTIM2 register block (&'static_mut)
pub unsafe fn lptim2_mut() -> &'static mut LowPowerTimer {
    deref_mut(LPTIM2)
}

/// USART1 register block (&'static)
pub fn usart1() -> &'static Usart {
    unsafe { deref(USART1) }
}

/// USART1 register block (&'static_mut)
pub unsafe fn usart1_mut() -> &'static mut Usart {
    deref_mut(USART1)
}

/// USART2 register block (&'static)
pub fn usart2() -> &'static Usart {
    unsafe { deref(USART2) }
}

/// USART2 register block (&'static_mut)
pub unsafe fn usart2_mut() -> &'static mut Usart {
    deref_mut(USART2)
}

/// USART3 register block (&'static)
pub fn usart3() -> &'static Usart {
    unsafe { deref(USART3) }
}

/// USART3 register block (&'static_mut)
pub unsafe fn usart3_mut() -> &'static mut Usart {
    deref_mut(USART3)
}

/// UART4 register block (&'static)
pub fn uart4() -> &'static Usart {
    unsafe { deref(UART4) }
}

/// UART4 register block (&'static_mut)
pub unsafe fn uart4_mut() -> &'static mut Usart {
    deref_mut(UART4)
}

/// UART5 register block (&'static)
pub fn uart5() -> &'static Usart {
    unsafe { deref(UART5) }
}

/// UART5 register block (&'static_mut)
pub unsafe fn uart5_mut() -> &'static mut Usart {
    deref_mut(UART5)
}

/// LPUART1 register block (&'static)
pub fn lpuart1() -> &'static Lpuart {
    unsafe { deref(LPUART1) }
}

/// LPUART1 register block (&'static_mut)
pub unsafe fn lpuart1_mut() -> &'static mut Lpuart {
    deref_mut(LPUART1)
}

/// SPI1 register block (&'static)
pub fn spi1() -> &'static Spi {
    unsafe { deref(SPI1) }
}

/// SPI1 register block (&'static_mut)
pub unsafe fn spi1_mut() -> &'static mut Spi {
    deref_mut(SPI1)
}

/// SPI2 register block (&'static)
pub fn spi2() -> &'static Spi {
    unsafe { deref(SPI2) }
}

/// SPI2 register block (&'static_mut)
pub unsafe fn spi2_mut() -> &'static mut Spi {
    deref_mut(SPI2)
}

/// SPI3 register block (&'static)
pub fn spi3() -> &'static Spi {
    unsafe { deref(SPI3) }
}

/// SPI3 register block (&'static_mut)
pub unsafe fn spi3_mut() -> &'static mut Spi {
    deref_mut(SPI3)
}

/// SDMMC register block (&'static)
pub fn sdmmc() -> &'static Sdmmc {
    unsafe { deref(SDMMC) }
}

/// SDMMC register block (&'static_mut)
pub unsafe fn sdmmc_mut() -> &'static mut Sdmmc {
    deref_mut(SDMMC)
}

/// EXTI register block (&'static)
pub fn exti() -> &'static Exti {
    unsafe { deref(EXTI) }
}

/// EXTI register block (&'static_mut)
pub unsafe fn exti_mut() -> &'static mut Exti {
    deref_mut(EXTI)
}

/// VREF register block (&'static)
pub fn vref() -> &'static Vref {
    unsafe { deref(VREF) }
}

/// VREF register block (&'static_mut)
pub unsafe fn vref_mut() -> &'static mut Vref {
    deref_mut(VREF)
}

/// CAN register block (&'static)
pub fn can() -> &'static Can {
    unsafe { deref(CAN) }
}

/// CAN register block (&'static_mut)
pub unsafe fn can_mut() -> &'static mut Can {
    deref_mut(CAN)
}

/// RTC register block (&'static)
pub fn rtc() -> &'static Rtc {
    unsafe { deref(RTC) }
}

/// RTC register block (&'static_mut)
pub unsafe fn rtc_mut() -> &'static mut Rtc {
    deref_mut(RTC)
}

/// OTG_FS_GLOBAL register block (&'static)
pub fn otg_fs_global() -> &'static OtgFsGlobal {
    unsafe { deref(OTG_FS_GLOBAL) }
}

/// OTG_FS_GLOBAL register block (&'static_mut)
pub unsafe fn otg_fs_global_mut() -> &'static mut OtgFsGlobal {
    deref_mut(OTG_FS_GLOBAL)
}

/// OTG_FS_HOST register block (&'static)
pub fn otg_fs_host() -> &'static OtgFsHost {
    unsafe { deref(OTG_FS_HOST) }
}

/// OTG_FS_HOST register block (&'static_mut)
pub unsafe fn otg_fs_host_mut() -> &'static mut OtgFsHost {
    deref_mut(OTG_FS_HOST)
}

/// OTG_FS_DEVICE register block (&'static)
pub fn otg_fs_device() -> &'static OtgFsDevice {
    unsafe { deref(OTG_FS_DEVICE) }
}

/// OTG_FS_DEVICE register block (&'static_mut)
pub unsafe fn otg_fs_device_mut() -> &'static mut OtgFsDevice {
    deref_mut(OTG_FS_DEVICE)
}

/// OTG_FS_PWRCLK register block (&'static)
pub fn otg_fs_pwrclk() -> &'static OtgFsPwrclk {
    unsafe { deref(OTG_FS_PWRCLK) }
}

/// OTG_FS_PWRCLK register block (&'static_mut)
pub unsafe fn otg_fs_pwrclk_mut() -> &'static mut OtgFsPwrclk {
    deref_mut(OTG_FS_PWRCLK)
}

/// SWPMI register block (&'static)
pub fn swpmi() -> &'static Swpmi {
    unsafe { deref(SWPMI1) }
}

/// SWPMI register block (&'static_mut)
pub unsafe fn swpmi_mut() -> &'static mut Swpmi {
    deref_mut(SWPMI1)
}

/// OPAMP register block (&'static)
pub fn opamp() -> &'static Opamp {
    unsafe { deref(OPAMP) }
}

/// OPAMP register block (&'static_mut)
pub unsafe fn opamp_mut() -> &'static mut Opamp {
    deref_mut(OPAMP)
}

/// FMC register block (&'static)
pub fn fmc() -> &'static Fmc {
    unsafe { deref(FMC) }
}

/// FMC register block (&'static_mut)
pub unsafe fn fmc_mut() -> &'static mut Fmc {
    deref_mut(FMC)
}

/// NVIC register block (&'static)
pub fn nvic() -> &'static Nvic {
    unsafe { deref(NVIC) }
}

/// NVIC register block (&'static_mut)
pub unsafe fn nvic_mut() -> &'static mut Nvic {
    deref_mut(NVIC)
}

// // Here we extend the peripheral API -- AKA ~~svd2rust is~~ SVD files are great
// // but not perfect
// use core::ptr;
//
// impl spi::Dr {
//     /// Reads a byte (`u8`) from this register
//     pub fn read_u8(&self) -> u8 {
//         unsafe { ptr::read_volatile(self as *const _ as *const u8) }
//     }
//
//     /// Writes a byte (`u8`) to this register
//     pub fn write_u8(&mut self, value: u8) {
//         unsafe { ptr::write_volatile(self as *mut _ as *mut u8, value) }
//     }
// }
//
