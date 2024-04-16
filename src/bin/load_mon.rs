// main.rs

use std::{thread, time};

use my_hacklab::*;

#[derive(Clone, Debug, Default, Parser)]
pub struct MyOpts {
    #[command(flatten)]
    c: OptsCommon,

    #[arg(long)]
    pub load: String,
}

fn main() -> anyhow::Result<()> {
    let opts = MyOpts::parse();
    opts.c.start_pgm(env!("CARGO_BIN_NAME"));
    debug!("config: {opts:?}");

    let mut load = SDL1000X::new("LOAD", &opts.load)?;
    //ld.verbose = true;
    info!("Lab LOAD at {:?}", load.lxi.addr());

    info!("***");
    thread::sleep(time::Duration::new(1, 0));

    loop {
        let volt = load.volt_m()?;
        let curr = load.curr_m()?;
        let pwr = load.powr_m()?;

        info!("Volt: {volt:.3}V Curr: {curr:.3}A Power: {pwr:.2}W");
        thread::sleep(time::Duration::new(10, 0));
    }
}

// EOF
