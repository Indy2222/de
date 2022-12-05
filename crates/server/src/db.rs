#[macro_export]
macro_rules! collision {
    ($result:expr, $error:expr) => {
        if let Err(sqlx::Error::Database(ref error)) = $result {
            if let Some(code) = error.code() {
                // This code corresponds to SQLITE_CONSTRAINT_PRIMARYKEY
                if code == "1555" {
                    return Err($error);
                }
            }
        }
    };
}
