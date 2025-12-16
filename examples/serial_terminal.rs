#![deny(unsafe_code)]
#![no_std]
#![no_main]

use core::{fmt::Write, str};
use cortex_m_rt::entry;
use nucleo_f446re::{Nucleo, led::LedDigital, serial::SerialPort};
use panic_probe as _;
use stm32f4xx_hal::{block, prelude::*};

#[entry]
fn main() -> ! {
    let mut nucleo = Nucleo::<LedDigital>::init().unwrap();
    let SerialPort { mut tx, mut rx } = nucleo.vcom;

    let greeting = [
        "Welcome to the Nucleo command center terminal!",
        "You can:",
        "\tblinky on",
        "\tblinky off",
    ];
    tx.write_str("\r\n").unwrap();
    for s in greeting {
        tx.write_str(s).unwrap();
        tx.write_str("\r\n").unwrap();
    }

    let mut line = [0_u8; 20];
    let mut line_idx = 0;
    let mut clear = true;
    loop {
        if clear {
            clear = false;
            line = [0_u8; 20];
            line_idx = 0;
            tx.write_str("\r\n? ").unwrap();
        }

        if let Ok(c) = block!(rx.read()) {
            let _ = block!(tx.write(c));

            if line_idx < line.len() {
                if c == b'\r' || c == b'\n' {
                    // Process command after a line ending is received

                    // Rust str's don't play nice with null terminations so subslice to remove it
                    let null_pos = line.iter().position(|&c| c == b'\0').unwrap_or(line.len()); // Default to line length if no null present
                    let cmd = &line[0..null_pos];

                    match str::from_utf8(cmd) {
                        Ok(s) => {
                            match s {
                                "blinky on" => {
                                    nucleo.user_led.on();
                                    tx.write_str("\r\nYou light up the room.").unwrap();
                                }
                                "blinky off" => {
                                    nucleo.user_led.off();
                                    tx
                                        .write_str("\r\nIt is pitch black. You are likely to be eaten by a grue.")
                                        .unwrap();
                                }
                                "" => (), // If sending two line endings in a row for example
                                _ => {
                                    write!(tx, "\r\nOops, invalid command: {:?}", s).unwrap();
                                }
                            }
                        }
                        Err(_) => {
                            write!(tx, "Oops, invalid str: {:?}", line).unwrap();
                        }
                    }
                    clear = true;
                } else if c == b'\x08' || c == b'\x7F' {
                    // Backspace
                    if line_idx > 0 {
                        line_idx -= 1;
                    }
                    line[line_idx] = 0;
                } else {
                    // Anything else
                    line[line_idx] = c;
                    line_idx += 1;
                }
            } else {
                tx.write_str("\r\nOops, line too long.").unwrap();
                clear = true;
            }
        } else {
            tx.write_str("\r\nOops, something went wrong reading a character.")
                .unwrap();
            clear = true;
        }
    }
}
