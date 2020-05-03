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

    // Set the color to green via a tuple and delay for 800ms
    let g = (0, 255, 0);
    bulb.set_tuple(g);
    bulb.delay_sec(0.8);

    // Turn off for 2 sec
    bulb.off()?;
    bulb.delay_msec(2000);


    // Turn on as red for 1sec
    let r = (255, 0, 0);
    bulb.set_tuple(r);
    bulb.delay_sec(1.);


    // Manually disconnect
    bulb.disconnect()?;

    Ok(())
}
```
