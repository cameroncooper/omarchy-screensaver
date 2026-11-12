//! Private-bus test publisher. Never opens desktop surfaces.
use omarchy_launch_screensaver::{engine::Animation, observer::Publisher, options::Options};
use std::io::{self, BufRead, Write};
fn main() -> anyhow::Result<()> {
    let publisher = Publisher::start();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    while let Some(line) = lines.next() {
        let effect = line?;
        if effect == "close" {
            drop(publisher);
            println!("closed");
            io::stdout().flush()?;
            let _ = lines.next(); // Keep the process alive while the bus name disappears.
            return Ok(());
        }
        if effect == "quit" {
            break;
        }
        let options = Options {
            effect: Some(effect),
            seed: Some(42),
            ..Options::default()
        };
        let mut animation = Animation::new("OMARCHY".into(), options, 669, 10)?;
        publisher.publish(animation.take_started_effect().unwrap());
        println!("published");
        io::stdout().flush()?;
    }
    Ok(())
}
