extern crate indicatif;

use std::thread;
use std::time::Duration;
use std::process::Command;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

/// ProgressBar factory function
fn progress(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("/|\\- ")
            .template("{spinner:.dim.bold} {wide_msg}"),
    );
    pb.enable_steady_tick(200);
    pb.set_message(msg);
    pb
}

/// Just runs a command. If `sleep` is not available this should be changed
/// to something else that runs for around 1-4 seconds.
/// If the command you chose runs longer than 3 seconds, adjust the Duration
/// in `use_multibar_with_timeout_for_finish()`.
fn run_command() {
    Command::new("sleep")
        .arg("3")
        .output()
        .unwrap();
    println!("done");
}

/// Run a single example that showcases indicatif behavior
fn run_example(description: &str, f: fn() -> ()) {
    println!("{}", description);
    thread::sleep(Duration::from_secs(2));

    f();

    println!("");
    println!("");
}

/// This example uses a multibar to display multiple spinners
/// These spinners will spin for 6 seconds before finishing, that means if the
/// command is running less than 6 seconds the two spinners will start spinning
/// right after the command finishes.
fn use_multibar_with_timeout_for_finish() {
    let m = MultiProgress::new();
    let pb = m.add(progress("multi #1"));
    let pb2 = m.add(progress("multi #2"));

    let _ = thread::spawn(move || {
        thread::sleep(Duration::from_millis(6000));
        pb.finish();
        pb2.finish();
    });

    run_command();

    m.join_and_clear().unwrap();
}

/// Use a multibar to display two spinners while the command runs. The spinners
/// should both spin while the command runs but they don't.
fn use_multibar() {
    let m = MultiProgress::new();
    let pb = m.add(progress("multi #1"));
    let pb2 = m.add(progress("multi #2"));

    run_command();

    pb.finish();
    pb2.finish();
    m.join_and_clear().unwrap();
}

/// A single spinner that happily spins while the command runs. This works fine except
/// that the spinner is duplicated, i.e. we end up with this output
/// 
/// ```
/// single spinner
/// - single #1
///   single #1
/// ```
/// 
/// where I would expect this to see
/// 
/// ```
/// single spinner
/// - single #1
/// ```
fn use_single_spinner() {
    let pb = progress("single #1");
    run_command();
    pb.finish();
}

fn main() {
    run_example("single spinner", use_single_spinner);

    run_example("multibar", use_multibar);

    run_example("multibar with time left after the subprocess - spinners gonna spin", use_multibar_with_timeout_for_finish);
}
