macro_rules! quit {
    ($string:expr) => {{
        println!("{}", $string);
        std::process::exit(1);
    }};
    () => {
        std::process::exit(0)
    };
}
