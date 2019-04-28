use crate::{config, effects, state, user};
use failure::Error;
use nanoleaf::client::Client;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use structopt::StructOpt;
use tokio::runtime::current_thread::Runtime;

#[derive(Debug, StructOpt)]
#[structopt(name = "nladm", about = "Control nanoleaf lighting")]
pub struct Opt {
    #[structopt(name = "server", long = "server", short = "s")]
    /// Server IP address
    pub server: Option<Ipv4Addr>,
    #[structopt(name = "port", long = "port", short = "p")]
    /// Server port
    pub port: Option<u16>,
    #[structopt(name = "token", long = "token", short = "t")]
    /// User token
    pub token: Option<String>,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "info")]
    /// Get panel information
    Info,
    #[structopt(name = "state")]
    /// Panel state
    State(state::State),
    #[structopt(name = "effects")]
    /// Panel effects
    Effects(effects::Effects),
    #[structopt(name = "user")]
    /// Add/Remove Users
    User(user::User),
}

pub fn execute() -> Result<(), Error> {
    let opt = Opt::from_args();
    let configuration = config::read_user_config()?;

    let ip = match opt.server {
        Some(server) => server,
        None => config::configure_server(configuration.as_ref())?,
    };

    let port = match opt.port {
        Some(port) => port,
        None => config::configure_port(configuration.as_ref())?,
    };

    // Left as a Result for API endpoint's that are unauthenticated
    let token = match opt.token {
        Some(token) => Ok(token),
        None => config::configure_token(configuration.as_ref()),
    };

    let client = Client::with_socketaddr(SocketAddr::new(std::net::IpAddr::V4(ip), port))?;
    let mut rt = Runtime::new().unwrap();
    match opt.cmd {
        Command::Info => {
            let token = token?;
            rt.block_on(client.get_panels(&token))
                .map(|i| println!("{:#?}", i))?;
        }
        Command::State(s) => {
            let token = token?;
            state::handle_state(s.state_type, client, &token, &mut rt)?;
        }
        Command::Effects(e) => {
            let token = token?;
            effects::handle_effect(e.effects_type, client, &token, &mut rt)?;
        }
        Command::User(u) => {
            user::handle_user(u.user_type, client, &mut rt)?;
        }
    }
    Ok(())
}
