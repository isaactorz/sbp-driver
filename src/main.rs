use std::error::Error;
use std::io;
use std::process;

use sbp::Sbp;

fn example() -> Result<(), Box<dyn Error>> {
    let messages = sbp::iter_messages(io::stdin());
    for msg in messages {
        let msg = msg?;
        if let Sbp::MsgObs(msg_obs) = msg {
            for ob in msg_obs.obs {
                let cn0 = ob.cn0;
                let sid = &ob.sid;
                let sat = sid.sat;
                eprintln!("SatID: {}, cn0: {}", sat, cn0);
            }
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
