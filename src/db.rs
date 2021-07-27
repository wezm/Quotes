use rocket_sync_db_pools::rusqlite;

use crate::Result;

pub fn migrate(conn: &mut rusqlite::Connection) -> Result<()> {
    embedded::migrations::runner().run(conn)?;
    Ok(())
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}
