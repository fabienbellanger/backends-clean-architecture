//! CLI

mod user;

use crate::api::axum_rest::server::start_server;
use clap::{Parser, Subcommand};
use clean_architecture_shared::error::CliResult;

#[derive(Parser)]
#[clap(
    name = crate::APP_NAME,
    version = clap::crate_version!(),
    author = clap::crate_authors!(),
)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start server
    #[clap(about = "Start Web server", long_about = None)]
    Serve,

    /// Register user
    #[clap(about = "Create a new user", long_about = None)]
    Register {
        /// User lastname
        #[clap(
            required = true,
            short = 'l',
            long,
            value_name = "Lastname",
            num_args = 1,
            help = "Lastname"
        )]
        lastname: String,

        /// User firstname
        #[clap(
            required = true,
            short = 'f',
            long,
            value_name = "Firstname",
            num_args = 1,
            help = "Firstname"
        )]
        firstname: String,

        /// User email
        #[clap(
            required = true,
            short = 'e',
            long,
            value_name = "Email",
            num_args = 1,
            help = "Email"
        )]
        email: String,

        /// User password (at least 8 characters)
        #[clap(
            required = true,
            short = 'p',
            long,
            value_name = "Password",
            num_args = 1,
            help = "Password (at least 8 characters)"
        )]
        password: String,
    },
}

/// Start CLI
pub async fn start() -> CliResult<()> {
    let args = Cli::parse();
    match &args.commands {
        Commands::Serve => start_server().await.map_err(|err| err.into()),
        Commands::Register {
            lastname,
            firstname,
            email,
            password,
        } => user::register(lastname, firstname, email, password).await,
    }
}
