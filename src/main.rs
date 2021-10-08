use std::error::Error;
use std::io;
use std::process;

use sbp::{SbpEncoder, SbpMessage};

fn example(sender_id: u16) -> Result<(), Box<dyn Error>> {
    let messages = sbp::iter_messages(io::stdin());
    let messages = messages.filter_map(|msg| match msg {
        Ok(msg) if msg.sender_id() == Some(sender_id) => Some(msg),
        _ => None,
    });
    SbpEncoder::new(io::stdout()).send_all(messages)?;
    Ok(())
}

fn main() {
    if let Err(err) = example(31183) {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}
