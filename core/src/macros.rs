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

        fn verbose(
            &self,
            input: &$lifetime $I,
        ) -> (Result<$O, $E>, Vec<$M>) {
            match self.apply::<$crate::Verbose>(input) {
                ModeResult::Success (output, messages) => (Ok (output), messages),
                ModeResult::Failure (error, messages) => (Err (error), messages),
            }
        }
    };
}