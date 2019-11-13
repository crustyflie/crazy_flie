//! On-board user LEDs

use crate::hal::prelude::*;

use crate::hal::gpio::gpioc::{self, PC, PC0, PC1, PC2, PC3};
use crate::hal::gpio::{Output, OpenDrain};

pub type LD1 = PC0<Output<OpenDrain>>;
pub type LD2 = PC1<Output<OpenDrain>>;
pub type LD3 = PC2<Output<OpenDrain>>;
pub type LD4 = PC3<Output<OpenDrain>>;

/// User LED colors
pub enum LedColor {
    Green,
    Red,
    Blue,
}

// Array of the on-board user LEDs
pub struct Leds {
    leds: [Led; 4],
}

impl Leds {
    pub fn new(gpioc: gpioc::Parts) -> Self {
        let left_red    = gpioc.pc0.into_open_drain_output();
        let left_green  = gpioc.pc1.into_open_drain_output();
        let right_green = gpioc.pc2.into_open_drain_output();
        let right_red   = gpioc.pc3.into_open_drain_output();

        Leds {
            leds: [left_green.into(), left_red.into(), right_green.into(), right_red.into()],
        }
    }
}

impl core::ops::Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &[Led] {
        &self.leds
    }
}

impl core::ops::DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}

impl core::ops::Index<usize> for Leds {
    type Output = Led;

    fn index(&self, i: usize) -> &Led {
        &self.leds[i]
    }
}

impl core::ops::Index<LedColor> for Leds {
    type Output = Led;

    fn index(&self, c: LedColor) -> &Led {
        &self.leds[c as usize]
    }
}

impl core::ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, i: usize) -> &mut Led {
        &mut self.leds[i]
    }
}

impl core::ops::IndexMut<LedColor> for Leds {
    fn index_mut(&mut self, c: LedColor) -> &mut Led {
        &mut self.leds[c as usize]
    }
}

/// One of the on-board user LEDs
pub struct Led {
    pin: PC<Output<OpenDrain>>,
}

macro_rules! ctor {
	($($ldx:ident),+) => {
		$(
			impl Into<Led> for $ldx {
				fn into(self) -> Led {
					Led {
						pin: self.downgrade(),
					}
				}
			}
		)+
	}
}

ctor!(LD1, LD2, LD3, LD4);

impl Led {
    /// Turns the LED off
    pub fn off(&mut self) {
        let _ = self.pin.set_low();
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        let _ = self.pin.set_high();
    }

    /// Toggles the LED
    pub fn toggle(&mut self) {
        if self.pin.is_low().unwrap() {
            let _ = self.pin.set_high();
        } else {
            let _ = self.pin.set_low();
        }
    }
}
