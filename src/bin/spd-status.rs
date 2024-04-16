// main.rs

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

    Ok(())
}
// EOF
