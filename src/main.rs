use std::error::Error;
use std::io;
use std::process;

use sbp::Sbp;

fn average_cn0(sat: u8) -> Result<(), Box<dyn Error>> {
    let mut acc: u64 = 0;
    let mut count: u64 = 0;
    let messages = sbp::iter_messages(io::stdin());
    for msg in messages {
        let msg = msg?;
        if let Sbp::MsgObs(msg_obs) = msg {
            for ob in msg_obs.obs {
                if sat == ob.sid.sat {
                    acc += ob.cn0 as u64;
                    count += 1;
                }
            }
        }
    }

    let average = acc as f64 / count as f64;
    println!("Average cn0 of Sat {}: {}", sat, average);
    Ok(())
}

fn main() {
    if let Err(err) = average_cn0(8) {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}
