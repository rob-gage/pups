// Copyright Rob Gage 2025

#[macro_export]
macro_rules! implement_modes {
    ($lifetime:lifetime, $O:ty, $E:ty, $M:ty, $I:ty) => {
        fn check(
            &self,
            input: &$lifetime $I,
        ) -> bool {
            self.apply::<$crate::Check>(input).is_success()
        }

        fn parse(
            &self,
            input: &$lifetime $I,
        ) -> $crate::ModeResult<$O, $E, $M, $crate::Parse> {
            self.apply::<$crate::Parse>(input)
        }
    };
}