# magiclight-rs

A Rust library for [MagicLight bulbs](https://smile.amazon.com/dp/B074VWLY1J).


```rust
use wifibulb_rust::*;

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

    // Set the color to red via a tuple and delay for 800ms
    let g = (0, 255, 0);
    bulb.set_tuple(g);
    bulb.delay_sec(1.);

    // Turn off for 1sec
    bulb.off()?;
    bulb.delay_msec(1000);


    // Turn on as red for 1sec
    let r = (255, 0, 0);
    bulb.set_tuple(r);
    bulb.delay_msec(1000);


    // Manually disconnect
    bulb.disconnect()?;

    Ok(())
}
```
