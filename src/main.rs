// #![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use cortex_m;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use hal::{
    gpio::*,
    interrupt,
    prelude::*,
    stm32
};

use core::cell::RefCell;
use cortex_m_semihosting::hprintln;

static MUTEX_PC13_BUTTON: Mutex<RefCell<Option<gpioc::PC13<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));
static MUTEX_PA5_LED: Mutex<RefCell<Option<gpioa::PA5<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
static MUTEX_EXTI:  Mutex<RefCell<Option<stm32::EXTI>>>  = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {    
    let board_peripherals = stm32::Peripherals::take().unwrap();

    let reset_and_clock_control = board_peripherals.RCC;
    // Enable the clock for peripherals on GPIOC
    reset_and_clock_control.ahb1enr.modify(|_, w| w.gpiocen().set_bit());
    // Enable the clock for peripherals in general
    reset_and_clock_control.apb2enr.modify(|_, w| w.syscfgen().set_bit());


    let gpioc = board_peripherals.GPIOC.split();
    let mut syscfg = board_peripherals.SYSCFG;
    let mut exti = board_peripherals.EXTI;
    let gpioa = board_peripherals.GPIOA.split();
    let pa5 = gpioa.pa5.into_push_pull_output();


    let mut pc13 = gpioc.pc13.into_pull_down_input();
    pc13.make_interrupt_source(&mut syscfg);
    pc13.trigger_on_edge(&mut exti, Edge::RISING_FALLING);
    pc13.enable_interrupt(&mut exti);
    
    
    cortex_m::interrupt::free(|cs| {
        MUTEX_PC13_BUTTON.borrow(cs).replace(Some(pc13));
        MUTEX_EXTI.borrow(cs).replace(Some(exti));
        MUTEX_PA5_LED.borrow(cs).replace(Some(pa5));
    });

    stm32::NVIC::unpend(stm32::interrupt::EXTI15_10);
    unsafe {
        stm32::NVIC::unmask(stm32::interrupt::EXTI15_10);
    }
    hprintln!("Entering loop").unwrap();
    loop {
        let button_state = cortex_m::interrupt::free(|cs| {          // enter critical section
            let pc13 = MUTEX_PC13_BUTTON.borrow(cs).borrow();   // acquire Mutex
            pc13.as_ref().unwrap().is_high().unwrap()           // read and return button state
        });
        cortex_m::interrupt::free(|_| {
            match button_state {
                true => hprintln!("loop: button state is true!").unwrap(),
                false => hprintln!("loop: button state is false!").unwrap(),
            }
        }); 
    }
}

#[interrupt]
fn EXTI15_10() {
    let button_state = cortex_m::interrupt::free(|cs| {
        let mut pc13 = MUTEX_PC13_BUTTON.borrow(cs).borrow_mut();
        let pc13 = pc13.as_mut().unwrap();
        // Reset the interrupt
        pc13.clear_interrupt_pending_bit();
        // Read the button state and return it.
        pc13.is_high().unwrap()           
    });

    cortex_m::interrupt::free(|cs| {
        match button_state {
            true => hprintln!("button state is true!").unwrap(),
            false => hprintln!("button state is false!").unwrap(),
        }
        let mut led = MUTEX_PA5_LED.borrow(cs).borrow_mut();
            led.as_mut().unwrap().toggle().unwrap();
    });    
}