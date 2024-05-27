mod cli;
mod process;
mod config;

pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use process::*;
pub use config::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self, settings: &Settings) -> anyhow::Result<()>;
}