// main.rs

use anyhow::anyhow;
use log::*;
use num::*;
use std::{thread, time};
use structopt::StructOpt;

use my_hacklab::*;

const LAB_LOAD: &str = "lab-load.siu.ro:5025";

const CURR_START: f32 = 0.010; // in A -- 10 mA
const CURR_LIMIT: f32 = 1.000; // in A
const DROP_MAX: f32 = 0.20; // 20%

fn main() -> anyhow::Result<()> {
    let opts = OptsCommon::from_args();
    start_pgm(&opts, "Load test");
    debug!("Global config: {opts:?}");

    let mut load = SDL1000X::new("LOAD", LAB_LOAD)?;
    //ld.verbose = true;
    info!("Lab LOAD at {:?}", load.lxi.addr());

    load.short_off()?;
    load.input_off()?;
    load.sense_on()?;

    info!("***");
    thread::sleep(time::Duration::new(1, 0));

    load.set_func(sdl1000x::Func::Curr)?;
    load.curr_irange(sdl1000x::IRange::I5A)?;
    load.curr_vrange(sdl1000x::VRange::V36V)?;

    load.lxi.req(":current:irange?")?;
    load.lxi.req(":current:vrange?")?;

    load.curr_curr(CURR_START)?;
    load.lxi.set_state(":input:state", PortState::On)?;

    info!("***");
    thread::sleep(time::Duration::new(2, 0));

    let volt_initial = load.volt_m()?;
    let volt_thres = volt_initial * (1.0 - DROP_MAX);
    let mut curr_step = CURR_START;
    let mut curr = CURR_START;

    while curr < CURR_LIMIT {
        curr += curr_step;
        curr_step *= 1.5;
        load.curr_curr(curr)?;

        thread::sleep(time::Duration::new(2, 0));
        load.res_m()?;
        load.curr_m()?;
        let pwr = load.powr_m()?;
        let volt = load.volt_m()?;
        let drop = 1.0 - volt / volt_initial;
        info!(
            "Curr: {curr:.3}A Volt: {volt:.3}V Power: {pwr:.2}W Drop: {drop_pct:.1}%",
            drop_pct = drop * 100.0
        );
        if volt < volt_thres {
            break;
        }
    }
    if curr > CURR_LIMIT {
        error!("Current limit {CURR_LIMIT:.3} A reached, cannot continue.");
        load.lxi.set_state(":input:state", PortState::Off)?;
        load.lxi.set_state("system:sense", PortState::Off)?;
        return Err(anyhow!("Current limit"));
    }
    while curr_step > CURR_START {
        // find the sweet spot with "CURR_START" accuracy
        info!("*** STEP: {curr_step:.3} A");
        let (stop_curr, steps) = steps_i(&mut load, volt_thres, curr, curr_step)?;
        info!("* took {steps} steps");
        curr = stop_curr;
        if steps < 2 {
            curr_step *= 0.5;
        }
    }

    let pwr = load.powr_m()?;
    let volt = load.volt_m()?;
    let drop = 1.0 - volt / volt_initial;
    info!(
        "Curr: {curr:.3}A Volt: {volt:.3}V Power: {pwr:.2}W Drop: {drop_pct:.1}%",
        drop_pct = drop * 100.0
    );

    load.lxi.set_state(":input:state", PortState::Off)?;
    load.lxi.set_state("system:sense", PortState::Off)?;

    Ok(())
}

/// return how many steps had to be taken to cross over threshold
fn steps_i(
    ld: &mut SDL1000X,
    v_thres: f32,
    i_start: f32,
    i_step: f32,
) -> anyhow::Result<(f32, usize)> {
    ld.curr_curr(i_start)?;
    thread::sleep(time::Duration::new(2, 0));
    let v_initial = ld.volt_m()?;

    // increase or decrease current?
    let i_sign = signum(v_initial - v_thres);

    let mut i_now = i_start;
    let mut n: usize = 0;
    loop {
        n += 1;
        i_now += i_sign * i_step;

        ld.curr_curr(i_now)?;
        thread::sleep(time::Duration::new(1, 0));
        let v_now = ld.volt_m()?;

        // did we cross the threshold?
        if (signum(v_now - v_thres) - i_sign).abs() > 0.1 {
            break;
        }
    }
    Ok((i_now, n))
}
// EOF
