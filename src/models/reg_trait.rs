use sqlx::sqlite::SqlitePool;

pub trait Reg: std::fmt::Debug {
    /*
     * Create a new register from a Vector of &str
     *
     * @param fields: Vec<&str> - The vector of strings
     * @return Self - The new register
     */
    fn new(fields: Vec<&str>) -> Self
    where
        Self: Sized;

    /*
     * Convert the register to a line of the file
     *
     * @return String - The line as a string
     */
    #[allow(dead_code)]
    fn to_line(&self) -> String {
        String::new()
    }

    async fn to_db(&self, reg: &Self, conn: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
        Ok(())
    }
}
