#!/bin/sh
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.
#
# Author: Jorge Aparicio <japaricious@gmail.com>
# Author: Brandon Edens <brandonedens@gmail.com>
# Date: 2016-11-19

set -ex

main() {
    local svd=STM32L4x6.svd

    if [ ! -f $svd ]; then
        curl -LOs https://github.com/posborne/cmsis-svd/raw/python-0.4/data/STMicro/$svd
    fi

    svd2rust -i $svd dac > src/dac.rs

    svd2rust -i $svd dma1 > src/dma.rs
    sed -i 's/\(pub struct \)Dma1/\1Dma/' src/dma.rs

    svd2rust -i $svd crc > src/crc.rs
    svd2rust -i $svd lcd > src/lcd.rs
    svd2rust -i $svd tsc > src/tsc.rs
    svd2rust -i $svd iwdg > src/iwdg.rs
    svd2rust -i $svd wwdg > src/wwdg.rs
    svd2rust -i $svd comp > src/comp.rs
    svd2rust -i $svd firewall > src/firewall.rs

    svd2rust -i $svd i2c1 > src/i2c.rs
    sed -i 's/\(pub struct I2c\)1/\1/' src/i2c.rs

    svd2rust -i $svd flash > src/flash.rs
    svd2rust -i $svd dbgmcu > src/dbgmcu.rs
    svd2rust -i $svd quadspi > src/quadspi.rs
    svd2rust -i $svd rcc > src/rcc.rs
    svd2rust -i $svd pwr > src/pwr.rs
    svd2rust -i $svd syscfg > src/syscfg.rs
    svd2rust -i $svd dfsdm > src/dfsdm.rs
    svd2rust -i $svd rng > src/rng.rs
    svd2rust -i $svd aes > src/aes.rs

    svd2rust -i $svd adc1 > src/adc.rs
    sed -i 's/\(pub struct Adc\)1/\1/' src/adc.rs

    svd2rust -i $svd adc123_common > src/adc_common.rs
    sed -i 's/\(pub struct \)Adc123Common/\1 AdcCommon/' src/adc_common.rs

    svd2rust -i $svd gpioa > src/gpio.rs
    sed -i 's/\(pub struct Gpio\)a/\1/' src/gpio.rs

    svd2rust -i $svd sai1 > src/sai.rs
    sed -i 's/\(pub struct \)Sai1/\1Sai/' src/sai.rs

    svd2rust -i $svd tim6 > src/basic_timer.rs
    sed -i 's/\(pub struct \)Tim6/\1BasicTimer/' src/basic_timer.rs

    svd2rust -i $svd tim2 > src/general_timer.rs
    sed -i 's/\(pub struct \)Tim2/\1GeneralTimer/' src/general_timer.rs

    svd2rust -i $svd tim15 > src/general_2channel_timer.rs
    sed -i 's/\(pub struct \)Tim15/\1General2ChannelTimer/' src/general_2channel_timer.rs

    svd2rust -i $svd tim16 > src/general_1channel_timer.rs
    sed -i 's/\(pub struct \)Tim16/\1General1ChannelTimer/' src/general_1channel_timer.rs

    svd2rust -i $svd tim1 > src/advanced_timer.rs
    sed -i 's/\(pub struct \)Tim1/\1AdvancedTimer/' src/advanced_timer.rs

    svd2rust -i $svd lptim1 > src/low_power_timer.rs
    sed -i 's/\(pub struct \)Lptim1/\1LowPowerTimer/' src/low_power_timer.rs

    svd2rust -i $svd usart1 > src/usart.rs
    sed -i 's/\(pub struct \)Usart1/\1Usart/' src/usart.rs

    svd2rust -i $svd lpuart1 > src/lpuart.rs
    sed -i 's/\(pub struct \)Lpuart1/\1Lpuart/' src/lpuart.rs

    svd2rust -i $svd spi1 > src/spi.rs
    sed -i 's/\(pub struct \)Spi1/\1Spi/' src/spi.rs

    svd2rust -i $svd sdmmc > src/sdmmc.rs
    svd2rust -i $svd exti > src/exti.rs
    svd2rust -i $svd vref > src/vref.rs
    svd2rust -i $svd can > src/can.rs
    svd2rust -i $svd rtc > src/rtc.rs
    svd2rust -i $svd otg_fs_global > src/otg_fs_global.rs
    svd2rust -i $svd otg_fs_host > src/otg_fs_host.rs
    svd2rust -i $svd otg_fs_device > src/otg_fs_device.rs
    svd2rust -i $svd otg_fs_pwrclk > src/otg_fs_pwrclk.rs

    svd2rust -i $svd swpmi1 > src/swpmi.rs
    sed -i 's/\(pub struct Swpmi\)1/\1/' src/swpmi.rs

    svd2rust -i $svd opamp > src/opamp.rs
    svd2rust -i $svd fmc > src/fmc.rs
    svd2rust -i $svd nvic > src/nvic.rs

    set +e
    rustfmt src/*.rs
    set -e

    xargo build --target thumbv7em-none-eabihf
}

main
