// main.rs

use log::*;
use std::{thread, time};
use structopt::StructOpt;

use my_hacklab::*;

const LAB_POWER: &str = "lab-power.siu.ro:5025";

fn main() -> anyhow::Result<()> {
    let opts = OptsCommon::from_args();
    start_pgm(&opts, "My Hacklab");
    debug!("Global config: {opts:?}");

    let mut pwr = SPD3303X::new("PWR", LAB_POWER)?;

    pwr.lxi.v_on();
    info!("Lab PWR at {:?}", pwr.lxi.addr());

    pwr.idn_q()?;
    pwr.version_q()?;
    pwr.error_q()?;

    pwr.lan_addr_q()?;
    pwr.lan_mask_q()?;
    pwr.lan_gw_q()?;

    info!("PWR status:\n{:#?}", pwr.status_q()?);

    thread::sleep(time::Duration::new(1, 0));

    loop {
        let volt = pwr.volt_m(Ch::Ch1)?;
        let curr = pwr.curr_m(Ch::Ch1)?;
        let pwr = pwr.powr_m(Ch::Ch1)?;

        info!("Volt: {volt:.3}V Curr: {curr:.3}A Power: {pwr:.2}W");
        thread::sleep(time::Duration::new(10, 0));
    }
}

// EOF
