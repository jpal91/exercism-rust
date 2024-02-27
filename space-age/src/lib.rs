// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_SECONDS: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration {
    years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        let s = s as f64;
        let years = s / EARTH_SECONDS;

        Self { years }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! impl_planet {
    ( $($planet:ident - $num:literal $(,)?)* ) => {
        paste::paste!{
            $(
                const [<$planet:upper _DURATION>]: f64 = $num;

                pub struct $planet;

                impl Planet for $planet {
                    fn years_during(d: &Duration) -> f64 {
                        d.years / [<$planet:upper _DURATION>]
                    }
                }
            )*
        }
    };
}

impl_planet!(
    Mercury - 0.2408467,
    Venus - 0.61519726,
    Mars - 1.8808158,
    Jupiter - 11.862615,
    Saturn - 29.447498,
    Uranus - 84.016846,
    Neptune - 164.79132,
    Earth - 1.0
);
