use std::io::{Write, BufWriter, stdout};
use rand::{thread_rng, Rng};
use std::time::SystemTime;
use terminal_size::{Width, Height, terminal_size};

const COUNT: usize = 30;
const RANGE: i64 = 100;

fn outstr(index: &i32, total: i64, scale: f64) -> String {
    let width: usize = (100.0 * scale).round() as usize;
    let current: usize = (((*index as f64 / total as f64) * 100.0) * scale).round() as usize;
    let percentage: usize = ((*index as f64 / total as f64) * 100.0).round() as usize;
    format!("[{}>{}] {}%", "=".repeat(current), " ".repeat(width - current), percentage)
}

fn check_complete(progress_bars: &[i32; COUNT]) -> bool {
    for i in progress_bars.iter() {
        let complete = match *i {
            i if i == 100 => true,
            _ => false
        };
        if !complete {
            return complete;
        }
    }
    true
}

fn get_incomplete_indexes(progress_bars: &[i32; COUNT]) -> Vec<i32> {
    let mut incomplete: Vec<i32> = vec![];
    for (i, bar) in progress_bars.iter().enumerate() {
        match *bar {
            b if b < 100 => incomplete.push(i as i32),
            _ => ()
        }
    }
    incomplete
}

fn main() {
    let start = SystemTime::now();
    let mut all_progress_bars: [i32; COUNT] = [0; COUNT];
    let mut rng = thread_rng();
    let size = terminal_size();
    let mut term_width: u16 = 0;
    if let Some((Width(w), Height(_h))) = size {
        term_width = w;
    }

    let mut count: usize = 0;

    let mut incomplete_indexes_timer: Vec<u128> = vec![];
    let mut random_gen_number: Vec<u128> = vec![];
    let mut print_time_count: Vec<u128> = vec![];

    let std = stdout();
    let lock = std.lock();
    let mut w = BufWriter::new(lock);

    write!(w, "{}", "\n".repeat(COUNT)).unwrap();
    while !check_complete(&all_progress_bars) {
        count += 1;
        let incomplete_start = SystemTime::now();
        let incomplete = get_incomplete_indexes(&all_progress_bars);
        match incomplete_start.elapsed() {
            Ok(elapsed) => incomplete_indexes_timer.push(elapsed.as_nanos()),
            _ => ()
        }

        let rnd_start = SystemTime::now();
        let rnd_index = rng.gen_range(0, incomplete.len());
        match rnd_start.elapsed() {
            Ok(elapsed) => random_gen_number.push(elapsed.as_nanos()),
            _ => ()
        }

        all_progress_bars[incomplete[rnd_index] as usize] += 1;
        write!(w, "\u{001b}[{}D", term_width).unwrap();
        write!(w, "\u{001b}[{}A", COUNT).unwrap();

        let print_start = SystemTime::now();
        for bar in all_progress_bars.iter() {
            write!(w, "{}\n", outstr(bar, RANGE, 1.0)).unwrap();
        }
        match print_start.elapsed() {
            Ok(elapsed) => print_time_count.push(elapsed.as_nanos()),
            _ => ()
        }

    }
    match start.elapsed() {
        Ok(elapsed) => {
            write!(w, "Time: {}s\n", elapsed.as_secs_f32()).unwrap();
            write!(w, "Iterations: {}\n", count).unwrap();
            write!(w, "Time per op: {}\n", elapsed.as_secs_f32() / count as f32).unwrap();
            let mut total: u128 = 0;
            for (_i, &val) in print_time_count.iter().enumerate() {
                total += val;
            }
        }
        Err(e) => {
            write!(w, "Error: {:?}\n", e).unwrap();
        }
    }
}
