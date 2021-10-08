use std::convert::TryFrom;
use std::error::Error;
use std::io;
use std::process;

use sbp::messages::logging::MsgLog;

fn example() -> Result<(), Box<dyn Error>> {
    let messages = sbp::iter_messages(io::stdin());
    for msg in messages {
        let msg = msg?; // ?
        match MsgLog::try_from(msg) {
            Ok(msg) => eprintln!("{}", msg.text),
            _ => {}
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}
