use diesel::sqlite::SqliteConnection;

pub struct Ctx<'a> {
    pub connection: &'a mut SqliteConnection,
    // optionally cache song paths if you want
    // pub songs: Option<Vec<String>>,
}