#[macro_export]
macro_rules! upgrade_or_break {
    ( $( $weak:ident ),* $(,)? ) => {
        let mut _success = true;
        $(
            let $weak = match $weak.upgrade() {
                Some(upgraded) => upgraded,
                None => {
                    _success = false;
                    break;
                }
            };
        )*
        if !_success {
            break;
        }
    };
}
