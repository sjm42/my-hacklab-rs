// main.rs

use std::{thread, time};

use my_hacklab::*;

#[derive(Clone, Debug, Default, Parser)]
pub struct MyOpts {
    #[command(flatten)]
    c: OptsCommon,

    #[arg(long)]
    pub power: String,
}


fn main() -> anyhow::Result<()> {
    let opts = MyOpts::parse();
    opts.c.start_pgm(env!("CARGO_BIN_NAME"));
    debug!("config: {opts:?}");

    let mut pwr = SPD3303X::new("PWR", &opts.power)?;

    pwr.lxi.v_on();
    info!("Lab PWR at {:?}", pwr.lxi.addr());

    pwr.idn_q()?;
    pwr.version_q()?;
    pwr.error_q()?;

    pwr.lan_addr_q()?;
    pwr.lan_mask_q()?;
    pwr.lan_gw_q()?;

    info!("PWR status:\n{:#?}", pwr.status_q()?);

    pwr.lxi.v_off();
    thread::sleep(time::Duration::new(1, 0));

    loop {
        let (curr1, curr2) = (pwr.curr_m(Ch::Ch1)?, pwr.curr_m(Ch::Ch2)?);
        let (volt1, volt2) = (pwr.volt_m(Ch::Ch1)?, pwr.volt_m(Ch::Ch2)?);
        let (pwr1, pwr2) = (pwr.powr_m(Ch::Ch1)?, pwr.powr_m(Ch::Ch2)?);
        let volt = volt1 + volt2;
        let pwr = pwr1 + pwr2;

        info!("***");
        info!("Volt: {volt:.3}V ({volt1:.3} + {volt2:.3}");
        info!("Curr: {curr1:.3}A + {curr2:.3}A");
        info!("Power: {pwr:.2}W ({pwr1:.2} + {pwr2:.2})");
        thread::sleep(time::Duration::new(10, 0));
    }
}

// EOF
