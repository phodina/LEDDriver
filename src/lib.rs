struct LEDDriver<'a>{
	address : &'a u32
}

impl <'a>LEDDriver<'a> {

	fn new(address : &'a u32) -> Self {
		LEDDriver { address : address}
	}
}

#[cfg(test)]
mod tests {

	use super::*;
    // TODO: All LEDs shall be off after the driver initialization.
    #[test]
    fn driver_init_leds_off () {
    	let addr = &0xffffffff;
    	let leddriver = LEDDriver::new(addr);
    	assert_eq!(0, *addr);
    }
    // TODO: A single LED can be turned on.
    // TODO: A single LED can be turned off.
    // TODO: Multiple LEDs can be turned on/off.
    // TODO: Turn on all LEDs.
    // TODO: Turn off all LEDs.
    // TODO: Query LED state.
    // TODO: Check boundary values.
    // TODO: Check out-of bounds values.
}
