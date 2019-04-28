use nanoleaf::client::Client;
use nanoleaf::error::Error;
use structopt::StructOpt;
use tokio::runtime::current_thread::Runtime;

#[derive(StructOpt, Debug)]
pub struct Effects {
    #[structopt(subcommand)]
    pub effects_type: EffectsType,
}

#[derive(StructOpt, Debug)]
pub enum EffectsType {
    #[structopt(name = "get")]
    /// get the current effect
    Get,
    #[structopt(name = "list")]
    /// List all effects
    List,
    #[structopt(name = "set")]
    /// Set an effect
    Set { effect: String },
}

pub fn handle_effect(
    effect: EffectsType,
    c: Client,
    t: &str,
    rt: &mut Runtime,
) -> Result<(), Error> {
    match effect {
        EffectsType::Get => rt.block_on(c.get_effect(t)).map(|v| println!("{}", v)),
        EffectsType::List => rt
            .block_on(c.get_all_effects(t))
            .map(|v| v.iter().for_each(|e| println!("{}", e))),
        EffectsType::Set { effect } => rt.block_on(c.set_effect(t, &effect)),
    }
}
