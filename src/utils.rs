#[macro_export]
macro_rules! x_printf {
    ( $std: ident,  $($t:tt)* ) => {
        {
            use std::io::Write;
            let mut h = std::io::$std();
            write!(h, $($t)* ).unwrap();
            h.flush().unwrap();
        }
    }
}

#[macro_export]
macro_rules! printf {
    ( $($t:tt)* ) => {
        x_printf!(stdout, $($t)*);
    }
}

#[macro_export]
macro_rules! eprintf {
    ( $($t:tt)* ) => {
        x_printf!(stderr, $($t)*);
    }
}