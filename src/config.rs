use failure::Error;
use serde::Deserialize;
use std::env;
use std::net::Ipv4Addr;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub server: Ipv4Addr,
    pub port: u16,
    pub token: String,
}

fn get_config() -> Option<PathBuf> {
    dirs::config_dir()
        .and_then(|mut c| {
            c.push("nladm.toml");
            Some(c)
        })
        .filter(|c| c.is_file())
}

pub fn read_user_config() -> Result<Option<Configuration>, Error> {
    if let Some(config) = get_config() {
        let contents = std::fs::read(config)?;
        return Ok(Some(toml::from_slice(&contents)?));
    }
    Ok(None)
}

// XXX it would be extremely nice to clean up all this code duplication with a macro or something

pub fn configure_server(config: Option<&Configuration>) -> Result<Ipv4Addr, Error> {
    if let Ok(server) = env::var("NANOLEAF_SERVER") {
        return Ok(server.parse()?);
    }
    if let Some(config) = config {
        return Ok(config.server);
    }

    Err(failure::err_msg(
        "server IP must be provided via --server, env variable (NANOLEAF_IP), or configuartion file",
    ))
}

pub fn configure_port(config: Option<&Configuration>) -> Result<u16, Error> {
    if let Ok(port) = env::var("NANOLEAF_PORT") {
        return Ok(port.parse()?);
    }
    if let Some(config) = config {
        return Ok(config.port);
    }

    Err(failure::err_msg(
        "server port must be provided via --port, env variable (NANOLEAF_PORT), or configuartion file",
    ))
}

pub fn configure_token(config: Option<&Configuration>) -> Result<String, Error> {
    if let Ok(token) = env::var("NANOLEAF_TOKEN") {
        return Ok(token.parse()?);
    }
    if let Some(config) = config {
        return Ok(config.token.clone());
    }

    Err(failure::err_msg(
        "server token must be provided via --token, env variable (NANOLEAF_TOKEN), or configuartion file",
    ))
}
