#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]

use super::BlockDevice;
use crate::sync::UPSafeCell;
use core::convert::TryInto;
use k210_hal::prelude::*;
use k210_pac::{Peripherals, SPI0};
use k210_soc::{
    fpioa::{self, io},
    //dmac::{dma_channel, DMAC, DMACExt},
    gpio,
    gpiohs,
    sleep::usleep,
    spi::{aitm, frame_format, tmod, work_mode, SPIExt, SPIImpl, SPI},
    sysctl,
};
use lazy_static::*;

pub struct SDCard<SPI> {
    spi: SPI,
    spi_cs: u32,
    cs_gpionum: u8,
    //dmac: &'a DMAC,
    //channel: dma_channel,
}

