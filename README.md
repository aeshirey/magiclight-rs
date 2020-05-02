# magiclight-rs

A Rust library for [MagicLight bulbs](https://smile.amazon.com/dp/B074VWLY1J).


```rust
use wifibulb_rust::*;

fn main() -> Result<(), WifiBulbError>{
    let mut bulb = WifiBulb::new("192.168.0.123")?;

    let blue = Color {
        red: 0,
        green: 0,
        blue: 255,
    };

    let r = (255, 0, 0);
    let g = (0, 255, 0);

    bulb.set_color(blue)?;

    bulb.delay_msec(1000);
    bulb.set_tuple(g);
    bulb.delay_msec(1000);
    bulb.off()?;
    bulb.delay_msec(1000);
    bulb.set_tuple(r);

    bulb.disconnect()?;

    Ok(())
}
```
