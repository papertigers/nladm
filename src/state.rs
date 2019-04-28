use nanoleaf::client::Client;
use nanoleaf::client::NanoleafState;
use nanoleaf::error::Error;
use structopt::StructOpt;
use tokio::runtime::current_thread::Runtime;

#[derive(StructOpt, Debug)]
pub struct State {
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub state_type: StateType,
}

#[derive(StructOpt, Debug)]
pub enum StateType {
    #[structopt(name = "get")]
    /// Get the current light state
    Get,
    #[structopt(name = "on")]
    /// Toggle the state on
    On,
    #[structopt(name = "off")]
    /// Toggle the state off
    Off,
}

pub fn handle_state(state: StateType, c: Client, t: &str, rt: &mut Runtime) -> Result<(), Error> {
    match state {
        StateType::Get => rt.block_on(c.get_state(t)).map(|v| {
            if v.value {
                println!("On");
            } else {
                println!("Off");
            }
        }),
        StateType::On => rt.block_on(c.set_state(t, NanoleafState::On)),
        StateType::Off => rt.block_on(c.set_state(t, NanoleafState::Off)),
    }
}
