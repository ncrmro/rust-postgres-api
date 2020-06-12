use clap::Clap;
use futures::executor::block_on;
use planet_express::core::auth::ViewerModel;
use planet_express::core::db::init_db;
use planet_express::core::settings::Settings;
use planet_express::init;
use planet_express::user::{NewUser, User};

#[derive(Clap)]
#[clap(version = "1.0", author = "Nicholas R. <ncrmro@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "1.0", author = "Nicholas R. <ncrmro@gmail.com>")]
    Createuser(Createuser),
    Runserver(Runserver),
}

#[derive(Clap)]
struct Runserver {
    #[clap(short, long)]
    _host: String,
    #[clap(short, long)]
    _port: String,
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct Createuser {
    #[clap(short, long)]
    email: String,
    #[clap(short, long)]
    password: String,
    // #[clap(short)]
    // superuser: Option<bool>,
}

//changeme
async fn create_user(args: Createuser, settings: Settings) {
    let conn = init_db(&settings.database).await.unwrap();
    let obj = NewUser {
        email: args.email,
        password: args.password,
    };

    let r = User::create_user(obj, &conn).await;
    match r {
        Ok(user) => println!("User successfully created {}", user.email),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    let settings = Settings::new().unwrap();

    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Createuser(args) => block_on(create_user(args, settings)),
        SubCommand::Runserver(_args) => init().unwrap(),
    }
}
