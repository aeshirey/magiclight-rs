#![allow(dead_code)]

use std::io::prelude::*;
use std::net::{Shutdown, TcpStream}; // SocketAddr when scanning
use std::thread;
use std::time::Duration;

const PORT_NUM: &str = ":5577";

#[derive(Debug)]
pub enum MagicLightError {
    IOError(String),
    Internal(String),
}

impl From<std::io::Error> for MagicLightError {
    // Os { code: 113, kind: Other, message: "No route to host" }
    fn from(e: std::io::Error) -> Self {
        println!("{:?}", e);
        MagicLightError::IOError("Some sort of IOError".to_string())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<(u8, u8, u8)> for Color {
    fn from(values: (u8, u8, u8)) -> Self {
        Color {
            red: values.0,
            green: values.1,
            blue: values.2,
        }
    }
}

#[derive(Debug)]
pub struct MagicLight {
    stream: TcpStream,
}

impl Drop for MagicLight {
    fn drop(&mut self) {
        self.disconnect().unwrap();
    }
}

impl MagicLight {
    fn scan(_timeout: Duration) -> Result<Self, MagicLightError> {
        todo!("Scan the local network for wifi bulbs")
    }

    pub fn new(address: &str) -> Result<Self, MagicLightError> {
        let mut full_addr = address.to_string();
        full_addr.push_str(PORT_NUM);

        println!("Connecting to {}", full_addr);

        let stream = TcpStream::connect(full_addr)?;

        Ok(MagicLight { stream })
    }

    pub fn disconnect(&mut self) -> Result<(), MagicLightError> {
        self.stream.shutdown(Shutdown::Both).map_err(|e| e.into())
    }

    pub fn off(&mut self) -> Result<(), MagicLightError> {
        self.set_color((0, 0, 0).into())?;
        Ok(())
    }

    pub fn set_color(&mut self, color: Color) -> Result<(), MagicLightError> {
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

        message[7] = MagicLight::checksum(&message);

        self.stream.write(&message)?;
        Ok(())
    }

    fn send_message(
        &mut self,
        rgb: (u8, u8, u8),
        extra: (u8, u8, u8),
    ) -> Result<(), MagicLightError> {
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

        message[7] = MagicLight::checksum(&message);

        self.stream.write(&message)?;
        Ok(())
    }

    fn checksum(byte_array: &[u8]) -> u8 {
        let mod_val: i32 = 2_i32.pow(byte_array.len() as u32 + 1);
        let sum: i32 = byte_array.iter().map(|&b| b as i32).sum::<i32>();
        (sum % mod_val) as u8
    }

    /// Convenience function for callers to delay.
    pub fn delay_sec(&self, sec: f32) {
        let dur = Duration::from_micros((sec * 1_000_000.) as u64);
        thread::sleep(dur);
    }

    /// Convenience function for callers to delay.
    pub fn delay_msec(&self, msec: u64) {
        let dur = Duration::from_millis(msec);
        thread::sleep(dur);
    }

    /// Fade between two colors.
    ///
    /// The light will fade from `from` to `to` over approximately the `duration` (plus time to execute color changes) in
    /// `num_steps` distinct steps.
    pub fn fade_between(&mut self, from: Color, to: Color, num_steps: usize, duration: Duration) {
        let sleep_time = Duration::from_millis(duration.as_millis() as u64 / num_steps as u64);

        let from_r = from.red as f32;
        let from_g = from.green as f32;
        let from_b = from.blue as f32;

        let to_r = to.red as f32;
        let to_g = to.green as f32;
        let to_b = to.blue as f32;

        let num_steps_f32 = num_steps as f32;

        for i in 0..=num_steps {
            // Figure out a blend of `from` and `to`
            let scale = i as f32 / num_steps_f32; // [0, 1]
            let step_r = (scale * to_r + (1.0 - scale) * from_r) as u8;
            let step_g = ((i as f32 * to_g / num_steps_f32)
                + ((num_steps - i) as f32 * from_g / num_steps_f32)) as u8;
            let step_b = ((i as f32 * to_b / num_steps_f32)
                + ((num_steps - i) as f32 * from_b / num_steps_f32)) as u8;

            let step_color = (step_r, step_g, step_b).into();
            self.set_color(step_color).unwrap();

            thread::sleep(sleep_time);
        }
    }

    /// Fades in from black
    pub fn fade_in(&mut self, to: Color, num_steps: usize, duration: Duration) {
        self.fade_between((0, 0, 0).into(), to, num_steps, duration);
    }

    /// Fades out to black
    pub fn fade_out(&mut self, from: Color, num_steps: usize, duration: Duration) {
        self.fade_between(from, (0, 0, 0).into(), num_steps, duration);
    }

    /// Blinks the specified color.
    pub fn blink(&mut self, color: Color, on_time: Duration, off_time: Duration, times: usize) {
        for _ in 0..times {
            self.set_color(color).unwrap();
            thread::sleep(on_time);
            self.off().unwrap();
            thread::sleep(off_time);
        }
    }
}
