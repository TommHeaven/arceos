

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_structs,register_bitfields,
    registers::ReadWrite,
};


register_structs!{
    pub BSTA1000BMUXIORegs{
        (0x00 => aon_pmm_reg_0: ReadWrite<u32>),
        (0x04 => aon_pmm_reg_1: ReadWrite<u32>),
        (0x08 => aon_pmm_reg_2: ReadWrite<u32>),
        (0x0c => aon_pmm_reg_3: ReadWrite<u32>),
        (0x10 => aon_pmm_reg_4: ReadWrite<u32>),
        (0x14 => _reserved),
        (0x50 => aon_io_reg_0: ReadWrite<u32>),
        (0x54 => aon_io_reg_1: ReadWrite<u32>),
        (0x58 => aon_io_reg_2: ReadWrite<u32>),
        (0x5c => aon_io_reg_3: ReadWrite<u32>),
        (0x60 => aon_io_reg_4: ReadWrite<u32>),
        (0x64 => aon_io_reg_5: ReadWrite<u32>),
        (0x68 => aon_io_reg_6: ReadWrite<u32>),
        (0x6c => aon_io_reg_7: ReadWrite<u32>),
        (0x70 => aon_io_reg_8: ReadWrite<u32>),
        (0x74 => aon_io_reg_9: ReadWrite<u32>),
        (0x78 => aon_io_reg_10: ReadWrite<u32>),
        (0x7c => aon_io_reg_11: ReadWrite<u32>),
        (0x80 => aon_io_reg_12: ReadWrite<u32>),
        (0x84 => aon_io_reg_13: ReadWrite<u32>),
        (0x88 => aon_io_reg_14: ReadWrite<u32>),
        (0x8c => aon_io_reg_15: ReadWrite<u32>),
        (0x90 => aon_io_reg_16: ReadWrite<u32>),
        (0x94 => aon_io_reg_17: ReadWrite<u32>),
        (0x98 => aon_io_reg_18: ReadWrite<u32>),
        (0x9c => aon_io_reg_19: ReadWrite<u32>),
        (0xa0 => aon_io_reg_20: ReadWrite<u32>),
        (0xa4 => aon_io_reg_21: ReadWrite<u32>),
        /// end
        (0xa8 => @END),
    }
}


register_bitfields!{u32,
    PAD_FNUC_0 [
        PAD_FUNC_0_0 OFFSET(0) NUMBITS(1) [
            A_GPIO_24 = 0,
            STRAP_R5_LOCK_STEP = 1,
        ],
        PAD_FUNC_0_1 OFFSET(1) NUMBITS(2) [
            A_GPIO_25 = 0,
            A_I2S0_SD_IN = 1,
            A_I2S1_SD_IN3 = 2 ,
            STRAP_R5_LOCK_STEP = 3,
        ],
        PAD_FUNC_0_3 OFFSET(3) NUMBITS(1) [
            A_GPIO_26 = 0,
            STRAP_IST_POST_AUTO_MODE = 1,
        ],
        PAD_FUNC_0_4 OFFSET(4) NUMBITS(1) [
            A_GPIO_27 = 0,
            STRAP_BOOT_MODE_SEL0 = 1,
        ],
        PAD_FUNC_0_5 OFFSET(5) NUMBITS(1) [
            A_GPIO_28 = 0,
            STRAP_BOOT_MODE_SEL1 = 1,
        ],
        PAD_FUNC_0_6 OFFSET(6) NUMBITS(2) [
            A_GPIO_29 = 0,
            A_I2S0_SD_IN2 = 1,
            A_I2S1_SD_OUT = 2,
            STRAP_RSV =3 ,
        ],
        PAD_FUNC_0_8 OFFSET(8) NUMBITS(2) [
            A_GPIO_30 = 0,
            A_I2S0_SD_IN3 = 1,
            A_I2S1_SD_OUT2 = 2,
            IST_POST_TEST_SEL0 =3 ,
        ],
        PAD_FUNC_0_10 OFFSET(10) NUMBITS(1) [
            A_JTAG_TMS = 0,
            GPIO_34 = 1,
        ],
        PAD_FUNC_0_11 OFFSET(11) NUMBITS(1) [
            A_JTAG_TDI = 0,
            GPIO_35 = 1,
        ],
        PAD_FUNC_0_12 OFFSET(12) NUMBITS(1) [
            A_JTAG_TDO = 0,
            GPIO_36 = 1,
        ],
        PAD_FUNC_0_13 OFFSET(13) NUMBITS(1) [
            A_UART0_TXD = 0,
            GPIO_37 = 1,
        ],
        PAD_FUNC_0_14 OFFSET(14) NUMBITS(1) [
            A_UART0_RXD = 0,
            GPIO_38 = 1,
        ],
        PAD_FUNC_0_15 OFFSET(15) NUMBITS(2) [
            A_UART0_CTS = 0,
            SPI2_SCLK = 1,
            I2C_SM0_SUS_INn = 2,
            GPIO_39 = 3,
        ],
        PAD_FUNC_0_17 OFFSET(17) NUMBITS(2) [
            A_GPIO_31 = 0,
            A_I2S0_SD_OUT3 = 1,
            A_I2S1_SD_OUT3 = 2,
            BOOT_FAIL = 3,
        ],
        PAD_FUNC_0_19 OFFSET(19) NUMBITS(2) [
            A_UART0_RTS = 0,
            SPI2_CS = 1,
            I2C_SM0_ALERT_INn = 2,
            GPIO_40 = 3,
        ],
        PAD_FUNC_0_21 OFFSET(21) NUMBITS(1) [
            A_UART2_TXD = 0,
            GPIO_45 = 1,
        ],
        PAD_FUNC_0_22 OFFSET(22) NUMBITS(1) [
            A_UART2_RXD = 0,
            GPIO_46 = 1,
        ],
    ],
}

pub struct Muxio {
    muxio_gpiob_base: usize,
}

impl Muxio {
    /// New a MUX_IO
    pub const fn new(muxio_gpiob_base: usize) -> Self {
        Self { muxio_gpiob_base }  
    }

    const fn mux_io_regs(&self) -> &BSTA1000BMUXIORegs {
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