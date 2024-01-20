#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use nucleo_f446re::{led::LedDigital, Nucleo};
use panic_probe as _;


use core::fmt::Write;
use rtt_target::{rtt_init, ChannelMode::BlockIfFull};

#[entry]
fn main() -> ! {
    let mut nucleo = Nucleo::<LedDigital>::init().unwrap();

    let channels = rtt_init! {
        up: {
            0: {
                size: 512,
                mode: BlockIfFull,
                name: "Up zero"
            }
            1: {
                size: 128,
                name: "Up one"
            }
            2: {
                size: 128,
                name: "Up two"
            }
        }
        down: {
            0: {
                size: 512,
                mode: BlockIfFull,
                name: "Down zero"
            }
        }
    };

    let mut output2 = channels.up.1;
    writeln!(
        output2,
        "Hi! I will turn anything you type on channel 0 into upper case."
    )
    .ok();

    let mut output = channels.up.0;
    let mut log = channels.up.2;
    let mut input = channels.down.0;
    let mut buf = [0u8; 512];
    let mut count: u8 = 0;

    loop {
        writeln!(log, "Top of loop, invoking input.read").ok();
        let bytes = loop {
            let bytes_read = input.read(&mut buf[..]);

            if bytes_read > 0 {
                break bytes_read;
            }

            writeln!(log, "No bytes read, sleeping").ok();
            nucleo.delay.delay_ms(1000_u32);
        };

        writeln!(log, "read {} bytes", bytes).ok();
        if bytes > 0 {
            for c in buf.iter_mut() {
                c.make_ascii_uppercase();
            }

            let mut p = 0;
            while p < bytes {
                p += output.write(&buf[p..bytes]);
            }
        }

        writeln!(log, "Messsge no. {}/{}", count, bytes).ok();
        nucleo.user_led.toggle();

        count += 1;

        nucleo.delay.delay_ms(10_u32);
        //for _ in 0..1_000_000 {
        //    cortex_m::asm::nop();
        //}
    }
}
