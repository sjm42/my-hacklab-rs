// main.rs

// use chrono::*;
use log::*;
use std::{error::Error, thread, time};
use structopt::StructOpt;

use my_hacklab::*;

const LAB_LOAD: &str = "lab-load.siu.ro:5025";

const CURR_START: f32 = 0.020; // 10 mA
const CURR_LIMIT: f32 = 1.000; // 5A
const DROP_MAX: f32 = 0.30; // 30%

fn main() -> Result<(), Box<dyn Error>> {
    let opts = OptsCommon::from_args();
    start_pgm(&opts, "Load test");
    debug!("Global config: {:?}", &opts);

    let mut ld = SDL1000X::new(LAB_LOAD, "LOAD".into())?;
    ld.verbose = true;
    info!("Lab LOAD at {:?}", ld.addr());

    ld.set_state(":short:state", PortState::Off)?;
    ld.set_state(":input:state", PortState::Off)?;
    ld.set_state("system:sense", PortState::On)?;

    info!("***");
    thread::sleep(time::Duration::new(1, 0));

    ld.set_func(sdl1000x::Func::Curr)?;
    ld.set(":current:irange", 5.0)?; // 5A or 30A
    ld.set(":current:vrange", 36.0)?; // 36V or 150V

    ld.req(":current:irange?")?;
    ld.req(":current:vrange?")?;

    ld.set(":current", CURR_START)?;
    ld.set_state(":input:state", PortState::On)?;

    info!("***");
    thread::sleep(time::Duration::new(2, 0));

    let volt_initial = ld.meas(sdl1000x::Meas::Volt)?;
    let mut curr_step = CURR_START;
    let mut curr = CURR_START;

    while curr < CURR_LIMIT {
        curr += curr_step;
        curr_step *= 1.5;
        ld.set(":current", curr)?;

        thread::sleep(time::Duration::new(2, 0));
        ld.meas(sdl1000x::Meas::Res)?;
        ld.meas(sdl1000x::Meas::Curr)?;
        let pwr = ld.meas(sdl1000x::Meas::Pow)?;
        let volt = ld.meas(sdl1000x::Meas::Volt)?;
        let drop = 1.0 - volt / volt_initial;
        info!(
            "Curr: {:.3}A Volt: {:.3}V Power: {:.2}W Drop: {:.1}%",
            curr,
            volt,
            pwr,
            drop * 100.0
        );
        if drop > DROP_MAX {
            break;
        }
    }
    ld.set_state(":input:state", PortState::Off)?;
    ld.set_state("system:sense", PortState::Off)?;

    Ok(())
}
// EOF
