#[macro_export]
macro_rules! point {
    ($($arg:tt)*) => ({
        use chrono::Utc;
        let now = Utc::now().format("%H:%M:%S");
        println!("[{}] {}", now, format_args!($($arg)*));
    })
}

pub fn startup() {
    
}