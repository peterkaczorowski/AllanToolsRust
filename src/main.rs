use std::env;
use std::error::Error;
use std::f64;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn frequency_to_phase(freq_data: &[f64], rate: f64) -> Vec<f64> {
    let dt = 1.0 / rate;
    let mean: f64 = freq_data.iter().sum::<f64>() / freq_data.len() as f64;
    let adjusted_freq: Vec<f64> = freq_data.iter().map(|&x| x - mean).collect();

    let mut phase_data = vec![0.0; adjusted_freq.len() + 1];
    for i in 1..phase_data.len() {
        phase_data[i] = phase_data[i - 1] + adjusted_freq[i - 1] * dt;
    }
    phase_data
}

fn input_to_phase(data: &[f64], rate: f64, data_type: &str) -> Result<Vec<f64>, String> {
    match data_type {
        "phase" => Ok(data.to_vec()),
        "freq" => Ok(frequency_to_phase(data, rate)),
        _ => Err(format!("Unknown data type: {}", data_type)),
    }
}

fn calc_adev_phase(
    phase: &[f64],
    rate: f64,
    mj: usize,
    stride: usize,
) -> Result<(f64, f64, usize), &'static str> {
    if phase.len() < 2 * mj {
        return Err("Data array length is too small.");
    }

    let d2: Vec<f64> = phase.iter().skip(2 * mj).step_by(stride).cloned().collect();
    let d1: Vec<f64> = phase.iter().skip(mj).step_by(stride).cloned().collect();
    let d0: Vec<f64> = phase.iter().step_by(stride).cloned().collect();

    let n = d0.len().min(d1.len()).min(d2.len());

    if n == 0 {
        return Err("Insufficient data for calculation.");
    }

    let s: f64 = d2
        .iter()
        .zip(d1.iter())
        .zip(d0.iter())
        .take(n)
        .map(|((&d2, &d1), &d0)| {
            let v = d2 - 2.0 * d1 + d0;
            v * v
        })
        .sum();

    let dev = (s / (2.0 * n as f64)).sqrt() / mj as f64 * rate;
    let deverr = dev / (n as f64).sqrt();
    Ok((dev, deverr, n))
}

fn remove_small_ns(
    taus: &[f64],
    devs: &[f64],
    deverrs: &[f64],
    ns: &[usize],
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<usize>) {
    let mut out_taus = vec![];
    let mut out_devs = vec![];
    let mut out_deverrs = vec![];
    let mut out_ns = vec![];

    for (i, &n) in ns.iter().enumerate() {
        if n > 1 {
            out_taus.push(taus[i]);
            out_devs.push(devs[i]);
            out_deverrs.push(deverrs[i]);
            out_ns.push(n);
        }
    }

    (out_taus, out_devs, out_deverrs, out_ns)
}

fn tau_generator(
    data: &[f64],
    rate: f64,
    taus_mode: Option<&str>,
    verbose: bool,
    even: bool,
    maximum_m: Option<usize>,
) -> Result<(Vec<f64>, Vec<usize>, Vec<f64>), Box<dyn Error>> {
    if rate == 0.0 {
        return Err("Warning! rate == 0".into());
    }

    let mut taus = Vec::new();

    match taus_mode {
        None | Some("octave") => {
            let maxn = (data.len() as f64).log2().floor() as usize;
            for i in 0..=maxn {
                taus.push(2f64.powi(i as i32) / rate);
            }
        }
        Some("log10") => {
            let maxn = (data.len() as f64).log10();
            let steps = (10.0 * maxn).round() as usize;
            for i in 0..steps {
                taus.push(10f64.powf(i as f64 / 10.0) / rate);
            }
            if verbose {
                println!("tau_generator: maxn = {}", maxn);
                for tau in &taus {
                    print!(" {}", tau);
                }
                println!();
            }
        }
        Some("decade") => {
            let maxn = (data.len() as f64).log10().floor() as usize;
            for k in 0..=maxn {
                taus.push(1.0 * 10f64.powi(k as i32) / rate);
                taus.push(2.0 * 10f64.powi(k as i32) / rate);
                taus.push(4.0 * 10f64.powi(k as i32) / rate);
            }
        }
        _ => (),
    }

    let mut m = Vec::new();
    for &tau in &taus {
        let mj = (tau * rate).round() as usize;
        if mj > 0 && mj < maximum_m.unwrap_or(data.len()) {
            m.push(mj);
        }
    }

    if even {
        m.retain(|&val| val % 2 == 0);
    }

    let taus_used: Vec<f64> = m.iter().map(|&mj| mj as f64 / rate).collect();

    if verbose {
        println!("tau_generator: m = {:?}", m);
    }

    Ok((taus, m, taus_used))
}

fn adev(
    data: Vec<f64>,
    rate: f64,
    data_type: &str,
    taus_mode: Option<&str>,
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<usize>), Box<dyn std::error::Error>> {

    let phase = input_to_phase(&data, rate, data_type)?;

    let (_taus, m, taus_used) = tau_generator(&phase, rate, taus_mode, false, false, None)?;

    let mut ad: Vec<f64> = vec![0.0; taus_used.len()];
    let mut ade: Vec<f64> = vec![0.0; taus_used.len()];
    let mut adn: Vec<usize> = vec![0; taus_used.len()];

    for (idx, &m_val) in m.iter().enumerate() {
        match calc_adev_phase(&phase, rate, m_val, m_val) {
            Ok((dev, deverr, n)) => {
                ad[idx] = dev;
                ade[idx] = deverr;
                adn[idx] = n;
            }
            Err(_e) => {
                // eprintln!("Error in calc_adev_phase for m[{}]: {}", idx, e);
                ad[idx] = 0.0;
                ade[idx] = 0.0;
                adn[idx] = 0;
            }
        }
    }

    Ok(remove_small_ns(&taus_used, &ad, &ade, &adn))
}

fn oadev(
    data: Vec<f64>,
    rate: f64,
    data_type: &str,
    taus_mode: Option<&str>,
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<usize>), Box<dyn std::error::Error>> {

    let phase = input_to_phase(&data, rate, data_type)?;

    let (_taus, m, taus_used) = tau_generator(&phase, rate, taus_mode, false, false, None)?;

    let mut ad: Vec<f64> = vec![0.0; taus_used.len()];
    let mut ade: Vec<f64> = vec![0.0; taus_used.len()];
    let mut adn: Vec<usize> = vec![0; taus_used.len()];

    for (idx, &m_val) in m.iter().enumerate() {
        match calc_adev_phase(&phase, rate, m_val, 1) { // "1" -> Overlapped ADEV
            Ok((dev, deverr, n)) => {
                ad[idx] = dev;
                ade[idx] = deverr;
                adn[idx] = n;
            }
            Err(_e) => {
                // eprintln!("Error in calc_adev_phase for m[{}]: {}", idx, e);
                ad[idx] = 0.0;
                ade[idx] = 0.0;
                adn[idx] = 0;
            }
        }
    }

    Ok(remove_small_ns(&taus_used, &ad, &ade, &adn))
}


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <inputFile> <samplePeriod> <dataType>", args[0]);
        return Err("Insufficient arguments".into());
    }

    let input_file = &args[1];
    let sample_period: f64 = args[2].parse()?;
    let data_type = &args[3];

    println!("# Input file: {}", input_file);
    println!("# Sample period: {}", sample_period);
    println!("# Data type: {}", data_type);

    if !Path::new(input_file).exists() {
        return Err("Input file not found.".into());
    }

    let file = File::open(input_file)?;
    let reader = io::BufReader::new(file);
    let data: Vec<f64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<f64>().unwrap())
        .collect();

    // Overlapped Allan Deviation
    let result = oadev(data, 1.0 / sample_period, data_type, None);

    match result {
        Ok((taus, adeviation, _, _)) => {
            for (tau, dev) in taus.iter().zip(adeviation.iter()) {
                println!("{:e} {:e}", tau, dev);
            }
        }
        Err(e) => {
            eprintln!("Error in oadev: {}", e);
        }
    }

    Ok(())
}
