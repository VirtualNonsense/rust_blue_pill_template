#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

// #################################################################################################
// Modules

// #################################################################################################
// Imports
use panic_halt as _;
use rtic::app;
use systick_monotonic::{Systick};

#[app(device = stm32f1xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use super::*;

    #[monotonic(binds = SysTick, default = true)]
    type HandleBufferMono = Systick<100>; // 100 Hz / 10 ms granularity

    #[shared]
    /// Critical sections are required to access #\[shared\] resources in a data race-free manner and to achieve this the shared field of the passed Context implements the Mutex trait for each shared resource accessible to the task.
    /// This trait has only one method, lock, which runs its closure argument in a critical section.
    /// The critical section created by the lock API is based on dynamic priorities:
    /// it temporarily raises the dynamic priority of the context to a ceiling priority that prevents other tasks from preempting the critical section.
    /// This synchronization protocol is known as the Immediate Ceiling Priority Protocol (ICPP),
    /// and complies with Stack Resource Policy (SRP) based scheduling of RTIC.
    ///
    /// [source](https://rtic.rs/dev/book/en/by-example/resources.html#shared-resources-and-lock)
    struct Shared {}

    #[local]
    /// #\[local\] resources are locally accessible to a specific task, meaning that only that task can access the resource and does so without locks or critical sections.
    /// This allows for the resources, commonly drivers or large objects, to be initialized in #\[init\] and then be passed to a specific task.
    /// Thus, a task #\[local\] resource can only be accessed by one singular task. Attempting to assign the same #\[local\] resource to more than one task is a compile-time error.
    /// Types of #\[local\] resources must implement Send trait as they are being sent from init to target task and thus crossing the thread boundary.
    ///
    /// [source](https://rtic.rs/dev/book/en/by-example/resources.html#local-resources)
    struct Local {
    }


    #[init()]
    // #############################################################################################
    ///An RTIC application requires an init task setting up the system.
    /// The corresponding init function must have the signature fn(init::Context)
    /// -> (Shared, Local, init::Monotonics),
    /// where Shared and Local are the resource structures defined by the user.
    ///The init task executes after system reset (after the optionally defined pre-init and internal
    /// RTIC initialization).
    /// The init task runs with interrupts disabled and has exclusive access to Cortex-M (the
    /// bare_metal::CriticalSection token is available as cs)
    /// while device specific peripherals are available through the core and device fields of
    /// init::Context.
    ///
    /// [source](https://rtic.rs/dev/book/en/by-example/app_init.html)
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // #########################################################################################
        // PERIPHERALS
        // Take ownership over the raw flash and rcc devices and convert them into the corresponding
        // HAL structs
        // let mut flash = cx.device.FLASH.constrain();
        // let rcc = cx.device.RCC.constrain();

        let systick = cx.core.SYST;

        // Initialize the monotonic (SysTick rate in QEMU is 12 MHz)
        let mono = Systick::new(systick, 12_000_000);

        // Freeze the configuration of all the clocks in the system and store the frozen frequencies
        // in `clocks`
        // let clocks = rcc.cfgr.freeze(&mut flash.acr);


        // Prepare the alternate function I/O registers
        // let mut afio = cx.device.AFIO.constrain();

        // preparing peripherals
        // let mut gpioa = cx.device.GPIOA.split();
        // let mut gpiob = cx.device.GPIOB.split();


        // #########################################################################################
        // resource structs
        // #########################################################################################
        (Shared {},
         Local {},
         init::Monotonics(mono))
    }
}