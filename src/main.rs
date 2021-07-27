#[macro_use]
extern crate rocket;

use gumdrop::Options;
use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::{database, rusqlite};

use quotes::db;

#[derive(Options)]
struct Args {
    #[options(help = "print help message")]
    help: bool,

    #[options(no_short, help = "run the database migrations")]
    migrate: bool,
}

#[database("quotes_db")]
struct QuotesDb(rusqlite::Connection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let args = Args::parse_args_default_or_exit();

    let db = AdHoc::on_ignite("Rusqlite Stage", |rocket| async {
        rocket
            .attach(QuotesDb::fairing())
            .attach(AdHoc::on_ignite("Rusqlite Init", move |rocket| {
                migrate(args.migrate, rocket)
            }))
            .mount("/rusqlite", routes![index])
    });

    rocket::build().attach(db).mount("/", routes![index])
}

async fn migrate(run_migrations: bool, rocket: Rocket<Build>) -> Rocket<Build> {
    // TODO: In a scenario where there are multiple servers, only one should run the migrations
    // and the others should wait for it.
    if run_migrations {
        QuotesDb::get_one(&rocket)
            .await
            .expect("unable to get db connection")
            .run(|conn| db::migrate(conn))
            .await
            .expect("migrations failed");
    }
    rocket
}
