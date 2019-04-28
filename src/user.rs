use nanoleaf::client::Client;
use nanoleaf::error::Error;
use structopt::StructOpt;
use tokio::runtime::current_thread::Runtime;

#[derive(StructOpt, Debug)]
pub struct User {
    #[structopt(subcommand)]
    pub user_type: UserType,
}

#[derive(StructOpt, Debug)]
pub enum UserType {
    #[structopt(name = "add")]
    /// Add a user (request must be sent within 30s of holding down the on-off button for 5-7s)
    Add,
    #[structopt(name = "del")]
    /// Delete a user by token
    Del { token: String },
}

pub fn handle_user(user: UserType, c: Client, rt: &mut Runtime) -> Result<(), Error> {
    match user {
        UserType::Add => rt.block_on(c.add_user()).map(|authorization| {
            println!(
                "Successfully added user:\n\t token: {}",
                authorization.auth_token,
            );
        }),
        UserType::Del { token } => rt.block_on(c.delete_user(&token)),
    }
}
