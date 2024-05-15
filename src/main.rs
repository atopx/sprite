mod core;

use tracing::{error, trace, Level};
use tracing_subscriber::FmtSubscriber;

use crate::core::interpreter::Interpreter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let mut args = std::env::args();

    if let Some(filename) = args.nth(1) {
        let mut interpreter = Interpreter::new();
        trace!("parse {}", filename);
        interpreter.parse_script(&filename)?;
        trace!("execute {}", filename);
        interpreter.execute();
    } else {
        error!("missing sprite script file, usage: sprite `script.spr`");
        std::process::exit(1);
    }
    Ok(())
}
