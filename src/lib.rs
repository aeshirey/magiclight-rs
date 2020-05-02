#![allow(dead_code)]

use std::io::prelude::*;
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::time::Instant;
use std::{thread, time};

const PORT_NUM: &'static str = ":5577";

#[derive(Debug)]
pub enum WifiBulbError {
    IOError(String),
    Internal(String),
}

impl From<std::io::Error> for WifiBulbError {
    // Os { code: 113, kind: Other, message: "No route to host" }
    fn from(e: std::io::Error) -> Self {
        println!("{:?}", e);
        WifiBulbError::IOError("Some sort of IOError".to_string())
    }
}

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug)]
pub struct WifiBulb {
    stream: TcpStream,
}

impl Drop for WifiBulb {
    fn drop(&mut self) {
        self.disconnect();
    }
}

impl WifiBulb {
    fn scan(timeout: std::time::Duration) -> Result<Self, WifiBulbError> {
        todo!()
    }

    pub fn new(address: &str) -> Result<Self, WifiBulbError> {
        let mut full_addr = address.to_string();
        full_addr.push_str(PORT_NUM);

        println!("Connecting to {}", full_addr);

        let stream = TcpStream::connect(full_addr)?;

        Ok(WifiBulb { stream })
    }

    pub fn disconnect(&mut self) -> Result<(), WifiBulbError> {
        self.stream.shutdown(Shutdown::Both).map_err(|e| e.into())
    }

    pub fn set_tuple(&mut self, colors: (u8, u8, u8)) -> Result<(), WifiBulbError> {
        self.set_color(Color {
            red: colors.0,
            green: colors.1,
            blue: colors.2,
        })?;

        Ok(())
    }

    pub fn off(&mut self) -> Result<(), WifiBulbError> {
        self.set_tuple((0, 0, 0))?;
        Ok(())
    }

    pub fn set_color(&mut self, color: Color) -> Result<(), WifiBulbError> {
        const MODE: u8 = 49; // corresponds with the ascii byte '1' == 0x31 == 49
                             // message is the sequence of bytes:
                             // send mode + red + green + blue + magic bytes + checksum
                             // 0x31 (49), <red>, <green>, <blue>, '00f00f', <checksum>
                             // note that '00f00f' is the string (not 0x00f00f!!), which is:
                             // 0x30 (48) + 0x30 (48) + 0x66 (102) + 0x30 (48) + 0x30 (48) + 0x66 (102)

        let mut message: [u8; 8] = [
            MODE,
            color.red,
            color.green,
            color.blue,
            0,
            240,
            15,
            0, // the checksum, to be added
        ];

        message[7] = WifiBulb::checksum(&message);

        self.stream.write(&message)?;
        Ok(())
    }

    fn send_message(
        &mut self,
        rgb: (u8, u8, u8),
        extra: (u8, u8, u8),
    ) -> Result<(), WifiBulbError> {
        const MODE: u8 = 49; // corresponds with the ascii byte '1' == 0x31 == 49
                             // message is the sequence of bytes:
                             // send mode + red + green + blue + magic bytes + checksum
                             // 0x31 (49), <red>, <green>, <blue>, '00f00f', <checksum>
                             // note that '00f00f' is the string (not 0x00f00f!!), which is:
                             // 0x30 (48) + 0x30 (48) + 0x66 (102) + 0x30 (48) + 0x30 (48) + 0x66 (102)

        let mut message: [u8; 8] = [
            MODE, rgb.0, rgb.1, rgb.2, extra.0, extra.1, extra.2,
            0, // the checksum, to be added
        ];

        message[7] = WifiBulb::checksum(&message);

        self.stream.write(&message)?;
        Ok(())
    }

    fn checksum(byte_array: &[u8]) -> u8 {
        let mod_val: i32 = 2_i32.pow(byte_array.len() as u32 + 1);
        let sum: i32 = byte_array.iter().map(|&b| b as i32).sum::<i32>();
        (sum % mod_val) as u8
    }

    pub fn delay_sec(&self, sec: f32) {
        let dur = time::Duration::from_micros((sec * 1_000_000.) as u64);
        thread::sleep(dur);
    }

    pub fn delay_msec(&self, msec: u64) {
        let dur = time::Duration::from_millis(msec);
        thread::sleep(dur);
    }
}
