use rand::{thread_rng, Rng};

const COUNT: usize = 30;
const RANGE: i64 = 100;

fn outstr(index: &i32, total: i64, scale: f64) -> String {
    let width: usize = (100.0 * scale).round() as usize;
    let current: usize= (((*index as f64 / total as f64) * 100.0) * scale).round() as usize;
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
    let mut all_progress_bars: [i32; COUNT] = [0; COUNT];
    let mut rng = thread_rng();
    print!("{}", "\n".repeat(COUNT));
    while !check_complete(&all_progress_bars) {
        let incomplete = get_incomplete_indexes(&all_progress_bars);
        let rnd_index = rng.gen_range(0, incomplete.len());
        all_progress_bars[incomplete[rnd_index] as usize] += 1;
        print!("\u{001b}[1000D");
        print!("\u{001b}[{}A", COUNT);

        for bar in all_progress_bars.iter() {
            print!("{}\n", outstr(bar, RANGE, 1.0));
        }
    }
}
