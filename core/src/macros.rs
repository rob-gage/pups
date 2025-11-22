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
        ) -> Result<$O, $E> {
            self.apply::<$crate::Parse>(input).to_result()
        }

        fn parse_verbose(
            &self,
            input: &$lifetime $I,
        ) -> $crate::ModeResult<$O, $E, $M, $crate::Verbose> {
            self.apply::<$crate::Verbose>(input)
        }
    };
}