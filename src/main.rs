use std::f64::consts::PI;
use std::io::BufRead;

const FULL_CIRCLE: f64 = 2.0 * PI;
const ONE_THIRD: f64 = FULL_CIRCLE / 3.0;
const TWO_THIRD: f64 = 2.0 * ONE_THIRD;

const N: usize = 180;
const STEP: f64 = FULL_CIRCLE / N as f64;

#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

const COLOR_RESET: &str = "\x1b[0m";

fn color_setter(color: Color) -> String {
    format!("\x1b[38;2;{};{};{}m", color.r, color.g, color.b)
}

fn color(fi: f64) -> Color {
    let fi = fi.rem_euclid(FULL_CIRCLE);
    let mut c = Color { r: 0, g: 0, b: 0 };

    let (x, y, fi) = if 0.0 <= fi && fi < ONE_THIRD {
        (&mut c.r, &mut c.g, fi)
    } else if ONE_THIRD <= fi && fi < TWO_THIRD {
        (&mut c.g, &mut c.b, fi - ONE_THIRD)
    } else {
        (&mut c.b, &mut c.r, fi - TWO_THIRD)
    };
    let fi = fi * 3.0 / 4.0;
    *x = (255.0 * fi.cos()).floor() as u8;
    *y = (255.0 * fi.sin()).floor() as u8;

    c
}

fn fill_color_table() -> Vec<String> {
    let mut table = Vec::with_capacity(N);
    for i in 0..N {
        let fi = i as f64 * STEP;
        let c: Color = color(fi);
        let s: String = color_setter(c);

        table.push(s);
    }
    table
}

fn step(fi: &mut usize) {
    *fi += 1;
    while *fi >= N {
        *fi = *fi - N;
    }
}

fn main() {
    let i = std::io::stdin();
    let mut i = i.lock();
    let mut line_buffer = String::new();
    let mut output_buffer = String::new();
    let color_table = fill_color_table();
    let mut idx = 0;
    loop {
        line_buffer.clear();
        let r = i.read_line(&mut line_buffer);
        match r {
            Ok(0) => break,
            Ok(_) => {
                step(&mut idx);
                let mut line_idx = idx.clone();
                output_buffer.clear();
                line_buffer.chars().for_each(|c| {
                    step(&mut line_idx);
                    output_buffer.push_str(&color_table[line_idx]);
                    output_buffer.push(c);
                    output_buffer.push_str(COLOR_RESET);
                });
                print!("{}", output_buffer);
            }
            Err(_) => continue,
        }
    }
}
