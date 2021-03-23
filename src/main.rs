use std::error::Error;
use std::io::{Read, BufRead};
use std::fs::File;
use std::time::Instant;

use deflate::deflate_bytes;

use plotters::prelude::*;

// use rayon::prelude::*;

fn main() -> Result<(), Box<dyn Error>>{
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut v = Vec::new();
    input.read_to_end(&mut v).expect("IO error");
    let ent = compute_ent(&v[..], 4096).expect("Error in computation");

    plot(&ent, &"plot.bmp").expect("Error plotting");

    Ok(())
}

fn compute_ent(data: &[u8], window: usize) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut result = Vec::new();

    let mut t0 = Instant::now();
    let mut last = 0;

    for (i, win) in data.windows(window).enumerate().step_by(32) {
        let compressed = deflate_bytes(&win[..]);
        result.push(compressed.len() as u32);

        let now = Instant::now();
        if (now - t0).as_millis() > 1000 {
            println!("{}\t({}M)", i - last, i / (1 << 20));
            last = i;
            t0 = now;
        }
    }
    Ok(result)
}

fn plot(data: &Vec<u32>, path: &str) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(path, (1600, 900)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        //.caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..data.len(), 0..4096usize)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            data.iter().enumerate().map(|(i, &x)| (i, x as usize)),
            &RED,
        ))?
        // .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}