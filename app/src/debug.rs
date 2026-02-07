#[macro_export]
macro_rules! debug_init {
    ($name:ident) => {
        #[cfg(debug_assertions)]
        let mut $name = 0;
    };
}

#[macro_export]
macro_rules! debug_break {
    ($count:ident, $limit:expr) => {
        #[cfg(debug_assertions)]
        {
            $count += 1;
            if $count >= $limit {
                break;
            }
        }
    };
}
