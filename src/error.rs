macro_rules! abort {
    () => {
        println!(message);
        std::exit
    };
}