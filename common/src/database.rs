use sqlite3::{
    DatabaseConnection,
    Query, ResultRow,
    ResultRowAccess,
    SqliteResult,
    StatementUpdate
}

struct Database {
    filename: String,
    decrypted: Vec<u8>,
    connection: DatabaseConnection
}

/* Define schemas here:
 *
 */

impl Database {

    pub fn new(filename: &str, props: &str) -> Database {
        let filename = filename.to_owned();
        // parse connection details
        let key = props.as_bytes();
        let cipher = Cipher::new(key);
        let bytes = std::fs::read(&filename)?;
        let decrypted = cipher.decrypt(bytes);
        let connection = DatabaseConnection::in_memory()?;
        Database {
            filename,
            decrypted,
            connection
        }
    }
}
