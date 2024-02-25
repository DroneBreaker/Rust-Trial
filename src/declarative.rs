pub fn declare() {
    macro_rules! average {
        ($(,)*) => {
            0.0
        };

        ( $($val: expr), + $(,)*) => {{
            let count = 0usize $(+ (let _ = stringify!($val); 1))*;

            let sum = 0.0 $(+ $val as f64)*;
            sum / count as f64
        }}
    }

    println!("{}", average!());
    println!("{}", average!(1, 0, 2, 0, 3, 0));
}