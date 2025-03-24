pub trait Reg: std::fmt::Debug {
    fn new(fields: Vec<&str>) -> Self
    where
        Self: Sized;

    // Convert the register to a line of the file
    #[allow(dead_code)]
    fn to_line(&self) -> String {
        String::new()
    }
}
