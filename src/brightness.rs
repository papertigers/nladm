use nanoleaf::client::Client;
use nanoleaf::error::Error;
use structopt::StructOpt;
use tokio::runtime::current_thread::Runtime;

#[derive(StructOpt, Debug)]
pub struct Brightness {
    #[structopt(subcommand)]
    pub brightness_type: Option<BrightnessType>,
}

#[derive(StructOpt, Debug)]
pub enum BrightnessType {
    #[structopt(name = "set")]
    /// Set the brightness of the panels
    Set {
        ///  Brightness level
        bri: u32,
        /// Change the brightness over n seconds
        #[structopt(name = "duration", long = "duration", short = "d")]
        duration: Option<u32>,
    },
    #[structopt(name = "incr")]
    /// Increase the brightness of the panels incrementally
    Incr {
        ///  Amount to increase the brightness
        amount: i32,
    },
    #[structopt(name = "decr")]
    /// Decrease the brightness of the panels incrementally
    Decr {
        ///  Amount to decrease the brightness
        amount: i32,
    },
}

pub fn handle_brightness(
    bri: Option<BrightnessType>,
    c: Client,
    t: &str,
    rt: &mut Runtime,
) -> Result<(), Error> {
    if let Some(bri) = bri {
        match bri {
            BrightnessType::Incr { amount } => {
                return rt.block_on(
                    c.set_brightness(t, nanoleaf::Brightness::Increment { increment: amount }),
                );
            }
            BrightnessType::Decr { amount } => {
                return rt.block_on(
                    c.set_brightness(t, nanoleaf::Brightness::Increment { increment: -amount }),
                );
            }
            BrightnessType::Set { bri, duration } => {
                let change = nanoleaf::Brightness::SetWithDuration {
                    value: bri,
                    duration: duration.unwrap_or_default(),
                };
                return rt.block_on(c.set_brightness(t, change));
            }
        }
    }

    rt.block_on(c.get_brightness(t))
        .map(|r| println!("{}", r.value))
}
