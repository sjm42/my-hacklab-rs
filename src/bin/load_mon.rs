// main.rs

use log::*;
use std::{thread, time};
use structopt::StructOpt;

use my_hacklab::*;

const LAB_LOAD: &str = "lab-load.siu.ro:5025";

fn main() -> anyhow::Result<()> {
    let opts = OptsCommon::from_args();
    start_pgm(&opts, "Load test");
    debug!("Global config: {opts:?}");

    let mut load = SDL1000X::new("LOAD", LAB_LOAD)?;
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
