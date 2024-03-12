//! led driver

use crate::mem::phys_to_virt;
use dw_apb_gpio::{Gpio, Muxio};
use memory_addr::PhysAddr;
use spinlock::SpinNoIrq;
use axconfig::{GPIO_PADDR,PINMUX_PADDR};


// 物理地址
const MUXIO_GPIOB_PHY: PhysAddr = PhysAddr::from(axconfig::PINMUX_PADDR); 
const GPIO_PHY: PhysAddr = PhysAddr::from(GPIO_PADDR); 

// 创建一个GPIO的SpinNoIrq
static GPIO: SpinNoIrq<Gpio> = 
SpinNoIrq::new(Gpio::new( phys_to_virt(GPIO_PHY).as_mut_ptr() ));

// 创建一个GPIO的SpinNoIrq
static MUXIO_GPIOB: SpinNoIrq<Muxio> = 
SpinNoIrq::new(Muxio::new( phys_to_virt(MUXIO_GPIOB_PHY).as_usize() ));

/// led simply initialize
pub fn init_led() {
    info!("Initialize Led...");
    MUXIO_GPIOB.lock().muxio_init();
    GPIO.lock().gpio_init();
}
