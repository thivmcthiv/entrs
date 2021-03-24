use std::error::Error;

use deflate::deflate_bytes;

use plotters::prelude::*;

use rayon::prelude::*;


pub fn par_compute_ent(data: &[u8], window: usize, step: usize) -> Result<Vec<(usize, f32)>, Box<dyn Error>> {
    let mut result = Vec::new();

    data.par_windows(window)
        .enumerate()
        .step_by(step)
        .map(|(i, win)| {
            let compressed = deflate_bytes(&win[..]);
            (i, compressed.len() as f32 / window as f32)
        })
        .collect_into_vec(&mut result);
    Ok(result)
}

// pub fn compute_ent(data: &[u8], window: usize, step: usize) -> Result<Vec<(usize, u32)>, Box<dyn Error>> {
//     let mut result = Vec::new();

//     let mut t0 = Instant::now();
//     let mut last = 0;

//     for (i, win) in data.windows(window).enumerate().step_by(step) {
//         let compressed = deflate_bytes(&win[..]);
//         result.push((i, compressed.len() as u32));

//         let now = Instant::now();
//         if (now - t0).as_millis() > 1000 {
//             println!("{}\t({}M)", i - last, i / (1 << 20));
//             last = i;
//             t0 = now;
//         }
//     }
//     Ok(result)
// }

pub fn plot(data: &Vec<(usize, f32)>, path: &str) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(path, (1600, 900)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Byte entropy (deflate)", ("sans-serif", 50).into_font())
        .margin(8)
        .x_label_area_size(32)
        .y_label_area_size(64)
        .build_cartesian_2d(0..data.last().unwrap().0, 0f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            data.iter().map(|&(i, x)| (i, x)),
            &RED,
        ))?;

    Ok(())
}