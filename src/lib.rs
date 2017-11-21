use std::cell::Cell;

enum LEDs {
    Led1,
    Led2,
    Led3,
    Led4,
    Led5,
    Led6,
    Led7,
    Led8,
    Led9,
    Led10,
    Led11,
    Led12,
    Led13,
    Led14,
    Led15,
    Led16,
}

struct LEDDriver<'a>{
	address: &'a Cell<u32>
}

impl <'a>LEDDriver<'a> {

	fn new(address: &'a Cell<u32>) -> Self {

		let leddriver = LEDDriver { address : address};
		leddriver.address.set(0);
		leddriver
	}

    fn turn_on (&mut self, led : LEDs) {
        match led {
            LEDs::Led1 => self.address.set(self.address.get() | 1<<0),
            LEDs::Led2 => self.address.set(self.address.get() | 1<<1),
            LEDs::Led3 => self.address.set(self.address.get() | 1<<2),
            LEDs::Led4 => self.address.set(self.address.get() | 1<<3),
            LEDs::Led5 => self.address.set(self.address.get() | 1<<4),
            LEDs::Led6 => self.address.set(self.address.get() | 1<<5),
            LEDs::Led7 => self.address.set(self.address.get() | 1<<6),
            LEDs::Led8 => self.address.set(self.address.get() | 1<<7),
            LEDs::Led9 => self.address.set(self.address.get() | 1<<8),
            LEDs::Led10 => self.address.set(self.address.get() | 1<<9),
            LEDs::Led11 => self.address.set(self.address.get() | 1<<10),
            LEDs::Led12 => self.address.set(self.address.get() | 1<<11),
            LEDs::Led13 => self.address.set(self.address.get() | 1<<12),
            LEDs::Led14 => self.address.set(self.address.get() | 1<<13),
            LEDs::Led15 => self.address.set(self.address.get() | 1<<14),
            LEDs::Led16 => self.address.set(self.address.get() | 1<<15),
        }
    }

    fn turn_off (&mut self, led : LEDs) {

        match led {
            LEDs::Led1 => self.address.set(self.address.get() & !(1<<0)),
            LEDs::Led2 => self.address.set(self.address.get() & !(1<<1)),
            LEDs::Led3 => self.address.set(self.address.get() & !(1<<2)),
            LEDs::Led4 => self.address.set(self.address.get() & !(1<<3)),
            LEDs::Led5 => self.address.set(self.address.get() & !(1<<4)),
            LEDs::Led6 => self.address.set(self.address.get() & !(1<<5)),
            LEDs::Led7 => self.address.set(self.address.get() & !(1<<6)),
            LEDs::Led8 => self.address.set(self.address.get() & !(1<<7)),
            LEDs::Led9 => self.address.set(self.address.get() & !(1<<8)),
            LEDs::Led10 => self.address.set(self.address.get() & !(1<<9)),
            LEDs::Led11 => self.address.set(self.address.get() & !(1<<10)),
            LEDs::Led12 => self.address.set(self.address.get() & !(1<<11)),
            LEDs::Led13 => self.address.set(self.address.get() & !(1<<12)),
            LEDs::Led14 => self.address.set(self.address.get() & !(1<<13)),
            LEDs::Led15 => self.address.set(self.address.get() & !(1<<14)),
            LEDs::Led16 => self.address.set(self.address.get() & !(1<<15)),
        }
    }
}

#[cfg(test)]
mod tests {

	use super::*;
    // REQ1: All LEDs shall be off after the driver initialization.
    #[test]
    fn initialization_leds_off () {

    	let ref mut addr = Cell::new(0xffffffff);
    	let leddriver = LEDDriver::new(addr);

    	assert_eq!(0, addr.get());
    }
    // REQ2: A single LED can be turned on.
    #[test]
    fn turn_on_led_one () {

        let ref mut addr = Cell::new(0x00000000);
        let mut leddriver = LEDDriver::new(addr);
        leddriver.turn_on(LEDs::Led1);
        assert_eq!(1, addr.get());

    }
    // REQ3: A single LED can be turned off.
    #[test]
    fn turn_off_led_one () {

        let ref mut addr = Cell::new(0xffffffff);
        let mut leddriver = LEDDriver::new(addr);
        leddriver.turn_on(LEDs::Led1);
        leddriver.turn_off(LEDs::Led1);
        assert_eq!(0, addr.get());

    }
    // REQ4: Multiple LEDs can be turned on.
    #[test]
    fn turn_on_multiple_leds () {

        let ref mut addr = Cell::new(0x00000000);
        let mut leddriver = LEDDriver::new(addr);
        leddriver.turn_on(LEDs::Led8);
        leddriver.turn_on(LEDs::Led9);
        assert_eq!(1<<7 | 1<<8, addr.get());
    }
    // REQ5: Multiple LEDs can be turned off.
    #[test]
    fn turn_off_multiple_leds () {

        let ref mut addr = Cell::new(0x00000000);
        let mut leddriver = LEDDriver::new(addr);
        leddriver.turn_on(LEDs::Led7);
        leddriver.turn_on(LEDs::Led8);
        leddriver.turn_on(LEDs::Led9);

        leddriver.turn_off(LEDs::Led7);
        leddriver.turn_off(LEDs::Led9);
        assert_eq!(1<<7, addr.get());
    }
    // TODO6: Turn on all LEDs.
    // TODO7: Turn off all LEDs.
    // TODO8: Query LED state.
    // TODO9: Check boundary values.
    // TODO10: Check out-of bounds values.
}
