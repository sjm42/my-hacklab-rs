// main.rs

use log::*;
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

    Ok(())
}
// EOF
