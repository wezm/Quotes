#[macro_use]
extern crate rocket;

use gumdrop::Options;
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

use ::quotes::db::{self, QuotesDb};
use quotes::quotes;

#[derive(Options)]
struct Args {
    #[options(help = "print help message")]
    help: bool,

    #[options(no_short, help = "run the database migrations")]
    migrate: bool,
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
    });

    rocket::build()
        .attach(db)
        .attach(Template::fairing())
        .mount("/", quotes::routes())
        .mount("/public", FileServer::from("public"))
}

async fn migrate(run_migrations: bool, rocket: Rocket<Build>) -> Rocket<Build> {
    // NOTE: In a scenario where there are multiple servers, only one should run the migrations
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
