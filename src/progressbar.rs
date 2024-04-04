use std::io::Write;
use std::{thread, time};

// create the function to be used from other .rs file
pub fn progressbar(total_steps: usize, desc: &str) {
    // ANSI escape codes for colors
    let green = "\x1B[32m";
    let reset = "\x1B[0m";

    let start_time = time::Instant::now(); // Start time of progress bar

    // Unicode symbols for the progress bar
    let filled_bar = "█";
    let empty_bar = "░";

    for i in 0..=total_steps {
        let progress = (i as f64 / total_steps as f64) * 100.0;
        let elapsed_time = start_time.elapsed().as_secs_f64(); // elapsed time in seconds

        let eta = if i > 0 {
            let average_time_per_step = elapsed_time / i as f64;
            let remaining_steps = total_steps - i;
            let remaining_time = average_time_per_step * remaining_steps as f64;
            format!("ETA: {:.1}s", remaining_time)
        } else {
            "".to_string()
        };
        print!(
            "Computing {}: [{}{}{}]",
            desc,
            green,
            filled_bar.repeat(i),
            empty_bar.repeat(total_steps - i)
        );
        print!(" {:.2}% ({})\r", progress, eta);

        let _ = std::io::stdout().flush();

        thread::sleep(time::Duration::from_millis(100));
    }

    let total_time = start_time.elapsed().as_secs_f64();
    println!(
        "\n{}{} completed! in {:.1} seconds!{}",
        green, desc, total_time, reset
    );
}
