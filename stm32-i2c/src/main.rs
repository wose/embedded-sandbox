#![no_std]

extern crate cortex_m;
extern crate cortex_m_semihosting as sh;
extern crate stm32f103xx_hal as hal;

extern crate bh1750;

use core::fmt::Write;

use sh::hio;
use hal::stm32f103xx;
use hal::prelude::*;
use hal::i2c::*;
use hal::i2c::DutyCycle::Ratio1to1;
use hal::delay::Delay;

use bh1750::BH1750;

fn main() {
    let mut hstdout = hio::hstdout().unwrap();

    if let Some(p) = stm32f103xx::Peripherals::take() {
        let mut flash = p.FLASH.constrain();
        let mut rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);
        let mut afio = p.AFIO.constrain(&mut rcc.apb2);

        let cp = cortex_m::Peripherals::take().unwrap();
        let mut delay = Delay::new(cp.SYST, clocks);

        /* Split GPIO pins */
        let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

        /* Prepare pins I2C1 is connected to */
        let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
        let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

        /* Setup I2C1 in fast mode */
        let mut i2c = I2c::i2c1(
            p.I2C1,
            (scl, sda),
            &mut afio.mapr,
            Mode::Fast {
                frequency: 400_000,
                duty_cycle: Ratio1to1,
            },
            clocks,
            &mut rcc.apb1,
        );

        let mut bh1750 = BH1750::new(i2c, delay);

        loop {
            match bh1750.illuminance() {
                Ok(value) => writeln!(hstdout, "{} lx", value).unwrap(),
                Err(e) => writeln!(hstdout, "{:?}", e).unwrap(),
            };
        }
    };
}
