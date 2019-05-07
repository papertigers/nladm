use futures::future::Future;
use nanoleaf::client::Client;
use nanoleaf::error::Error;
use structopt::StructOpt;
use tokio::runtime::current_thread::Runtime;

#[derive(StructOpt, Debug)]
pub struct Effects {
    #[structopt(subcommand)]
    pub effects_type: Option<EffectsType>,
}

#[derive(StructOpt, Debug)]
pub enum EffectsType {
    #[structopt(name = "list")]
    /// List all effects
    List,
    #[structopt(name = "set")]
    /// Set an effect
    Set {
        /// Name of the Effect
        effect: String,
        /// Brightness level
        #[structopt(name = "brightness", long = "brightness", short = "b")]
        brightness: Option<u32>,
    },
}

pub fn handle_effect(
    effect: Option<EffectsType>,
    c: Client,
    t: &str,
    rt: &mut Runtime,
) -> Result<(), Error> {
    if let Some(effect) = effect {
        match effect {
            EffectsType::List => {
                return rt
                    .block_on(c.get_all_effects(t))
                    .map(|v| v.iter().for_each(|e| println!("{}", e)))
            }
            EffectsType::Set { effect, brightness } => {
                let effect = c.set_effect(t, &effect);
                if let Some(b) = brightness {
                    let bri = nanoleaf::Brightness::Set { value: b };
                    return rt.block_on(effect.join(c.set_brightness(t, bri)).map(|_| ()));
                }
                return rt.block_on(effect);
            }
        }
    }

    rt.block_on(c.get_effect(t)).map(|v| println!("{}", v))
}
