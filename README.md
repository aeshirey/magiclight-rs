# magiclight-rs

A Rust library for [MagicLight bulbs](https://smile.amazon.com/dp/B074VWLY1J).


```rust
use wifibulb_rust::*;
use std::time::Duration;

fn main() -> Result<(), WifiBulbError>{
    let mut bulb = WifiBulb::new("192.168.0.123")?;

    // Set the color to blue and delay for 1000ms
    let blue = Color {
        red: 0,
        green: 0,
        blue: 255,
    };
    bulb.set_color(blue)?;
    bulb.delay_msec(1000);

    // Set the color to green via a tuple and delay for 800ms
    let g = (0, 255, 0).into();
    bulb.set_color(g);
    bulb.delay_sec(0.8);

    // Turn off for 2 sec
    bulb.off()?;
    bulb.delay_msec(2000);


    // Turn on as red for 1sec
    let r = (255, 0, 0).into();
    bulb.set_color(r);
    bulb.delay_sec(1.);

    // Fade from red to blue in five seconds
    bulb.fade_between(r, blue, 100, Duration::from_secs(5));

    // Then blink ten times, on 0.5s, off 0.25s
    bulb.blink(blue, Duration::from_secs_f32(0.5), Duration::from_secs_f32(0.25), 10);

    // And finally, fade out and disconnect
    bulb.fade_out(blue, 10, Duration::SECOND);
    bulb.disconnect()?;

    Ok(())
}
```
