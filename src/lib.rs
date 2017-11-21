use std::cell::Cell;

struct LEDDriver<'a>{
	address: &'a Cell<u32>
}

impl <'a>LEDDriver<'a> {

	fn new(address: &'a Cell<u32>) -> Self {

		let leddriver = LEDDriver { address : address};
		leddriver.address.set(0);
		leddriver
	}
}

#[cfg(test)]
mod tests {

	use super::*;
    // TODO: All LEDs shall be off after the driver initialization.
    #[test]
    fn driver_init_leds_off () {

    	let ref mut addr = Cell::new(0xffffffff);
    	let leddriver = LEDDriver::new(addr);

    	assert_eq!(0, addr.get());
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
