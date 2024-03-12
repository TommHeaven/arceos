//! gpio-led
//! 
#![no_std]
#![feature(const_ptr_as_ref)]
#![feature(const_option)]
#![feature(const_nonnull_new)]


use core::ptr::NonNull;

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_structs,
    registers::ReadWrite,
};

register_structs!{
    pub GpioRegisters{
        (0x00 => gpio_swporta_dr: ReadWrite<u32>),
        (0x04 => gpio_swporta_ddr: ReadWrite<u32>),
        (0x08 => gpio_swporta_ctl: ReadWrite<u32>),
        (0x0c => gpio_swportb_dr: ReadWrite<u32>),
        (0x10 => gpio_swportb_ddr: ReadWrite<u32>),
        (0x14 => gpio_swportb_ctl: ReadWrite<u32>),
        (0x18 => gpio_swportc_dr: ReadWrite<u32>),
        (0x1c => gpio_swportc_ddr: ReadWrite<u32>),
        (0x20 => gpio_swportc_ctl: ReadWrite<u32>),
        (0x24 => gpio_swportd_dr: ReadWrite<u32>),
        (0x28 => gpio_swportd_ddr: ReadWrite<u32>),
        (0x2c => gpio_swportd_ctl: ReadWrite<u32>),
        /// end
        (0x30 => @END),
    },
    pub MUXIORegs{
        (0x00 => aon_pmm_reg_0: ReadWrite<u32>),
        (0x04 => aon_pmm_reg_1: ReadWrite<u32>),
        (0x08 => aon_pmm_reg_2: ReadWrite<u32>),
        (0x0c => aon_pmm_reg_3: ReadWrite<u32>),
        (0x10 => aon_pmm_reg_4: ReadWrite<u32>),
        /// end
        (0x14 => @END),
    }
}

unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}


pub struct Gpio{
    registers: NonNull<GpioRegisters>,
}

pub struct Muxio {
    muxio_gpiob_base: usize,
}

impl Gpio {
    /// New a Gpio
    pub const fn new( registers:*mut u8 ) -> Self {
        Self { registers:NonNull::new(registers).unwrap().cast(),
        }
    }

    const fn gpio_regs(&self) -> &GpioRegisters {
        unsafe { self.registers.as_ref() }
    }

    /// GPIO initialize
    pub fn gpio_init(&mut self) {
        // init gpio
        /* set ddr bit17,18,19,20置1,设置为输出*/
        self.gpio_regs().gpio_swportb_ddr.set(self.gpio_regs().gpio_swportb_ddr.get() | (0xf << 17));

        /* set dr bit17,18置1,打开LED */
        self.gpio_regs().gpio_swportb_dr.set(self.gpio_regs().gpio_swportb_dr.get() & !(0xf << 17));
    }

    /// GPIO 引脚写入电平值,pin为引脚号,val为电平值
    pub fn gpio_write(&mut self, pin:u8, val:u8) {
        let flag = pin / 32 ;
        let offset = pin % 32 ;

        if val == 1 {
            match flag {
                0 =>  self.gpio_regs().gpio_swporta_dr.set(self.gpio_regs().gpio_swporta_dr.get() | (1 << (offset))),
                1 =>  self.gpio_regs().gpio_swportb_dr.set(self.gpio_regs().gpio_swportb_dr.get() | (1 << (offset))),
                2 =>  self.gpio_regs().gpio_swportc_dr.set(self.gpio_regs().gpio_swportc_dr.get() | (1 << (offset))),
                3 =>  self.gpio_regs().gpio_swportd_dr.set(self.gpio_regs().gpio_swportd_dr.get() | (1 << (offset))),
                _ => (),
            }
        }
        else {
            match flag {
                0 =>  self.gpio_regs().gpio_swporta_dr.set(self.gpio_regs().gpio_swporta_dr.get() & ! (1 << (offset))),
                1 =>  self.gpio_regs().gpio_swportb_dr.set(self.gpio_regs().gpio_swportb_dr.get() & ! (1 << (offset))),
                2 =>  self.gpio_regs().gpio_swportc_dr.set(self.gpio_regs().gpio_swportc_dr.get() & ! (1 << (offset))),
                3 =>  self.gpio_regs().gpio_swportd_dr.set(self.gpio_regs().gpio_swportd_dr.get() & ! (1 << (offset))),
                _ => (),
            }
        }
    }

    // GPIO 引脚读取电平值，pin为引脚号,val为电平值
    pub fn gpio_read(&mut self, pin:u8) -> u8 {
        let offset = pin % 32 ;
        ((self.gpio_regs().gpio_swportb_dr.get() >> offset) & 1  ) as u8
    }

    /// GPIO 设置GPIO引脚输入/输出方向,pin为引脚号
    pub fn gpio_setdir(&mut self, pin:u8, val:u8) {
        let flag = pin / 32 ;
        let offset = pin % 32 ;
        if val == 1 {
            match flag {
                0 =>  self.gpio_regs().gpio_swporta_ddr.set(self.gpio_regs().gpio_swporta_ddr.get() | (1 << (offset))),
                1 =>  self.gpio_regs().gpio_swportb_ddr.set(self.gpio_regs().gpio_swportb_ddr.get() | (1 << (offset))),
                2 =>  self.gpio_regs().gpio_swportc_ddr.set(self.gpio_regs().gpio_swportc_ddr.get() | (1 << (offset))),
                3 =>  self.gpio_regs().gpio_swportd_ddr.set(self.gpio_regs().gpio_swportd_ddr.get() | (1 << (offset))),
                _ => (),
            }
        }
        else {
            match flag {
                0 =>  self.gpio_regs().gpio_swporta_ddr.set(self.gpio_regs().gpio_swporta_ddr.get() & ! (1 << (offset))),
                1 =>  self.gpio_regs().gpio_swportb_ddr.set(self.gpio_regs().gpio_swportb_ddr.get() & ! (1 << (offset))),
                2 =>  self.gpio_regs().gpio_swportc_ddr.set(self.gpio_regs().gpio_swportc_ddr.get() & ! (1 << (offset))),
                3 =>  self.gpio_regs().gpio_swportd_ddr.set(self.gpio_regs().gpio_swportd_ddr.get() & ! (1 << (offset))),
                _ => (),
                }
        }
    }

    // GPIO 读GPIO引脚输入/输出方向
    pub fn gpio_getdir(&mut self, pin:u8) -> u8 {
        let offset = pin % 32 ;
        ((self.gpio_regs().gpio_swportb_ddr.get() >> offset) & 1  ) as u8
    }

}


impl Muxio {
    /// New a MUX_IO
    pub const fn new(muxio_gpiob_base: usize) -> Self {
        Self { muxio_gpiob_base }  
    }

    const fn mux_io_regs(&self) -> &MUXIORegs {
        unsafe { &*(self.muxio_gpiob_base as *const _) }
    }

    /// MUXIO initialize
    pub fn muxio_init(&mut self) {
        // init led
        /* gpio49,50   bit25 bit26 set 1,设置为GPIO模式 */
        self.mux_io_regs().aon_pmm_reg_0.set(self.mux_io_regs().aon_pmm_reg_0.get() | (3 << 25));

        /* gpio51,52   offset为0010 bit20,bit21,bit22,bit23 set 1,设置为GPIO模式 */
        self.mux_io_regs().aon_pmm_reg_4.set(self.mux_io_regs().aon_pmm_reg_4.get() | (0xf << 20));
    }

}