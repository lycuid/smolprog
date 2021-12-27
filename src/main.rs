mod loggers;

use loggers::{create_loggers, Logger};
use std::{
    fs,
    io::{self, Write},
    sync::mpsc,
    thread,
    time::Duration,
};

fn spawn_thread(
    index: usize,
    logger: Logger,
    sender: mpsc::Sender<(usize, String)>,
) -> thread::JoinHandle<()> {
    match logger {
        Logger::ValueLogger {
            default_value,
            interval_ms,
            create_runner,
        } => thread::spawn(move || {
            let mut runner = create_runner();
            sender.send((index, default_value.clone())).unwrap();

            loop {
                let value = runner.get_value().unwrap_or(default_value.clone());
                sender.send((index, value)).unwrap();
                thread::sleep(Duration::from_millis(interval_ms));
            }
        }),

        Logger::FifoLogger {
            default_value,
            fifopath,
            create_runner,
        } => thread::spawn(move || {
            let mut runner = create_runner();
            let fmt_defvalue = runner.fmt_value(&default_value);
            sender.send((index, fmt_defvalue.clone())).unwrap();

            loop {
                if let Ok(data) = fs::read_to_string(&fifopath) {
                    let line = data.lines().next().unwrap_or(&default_value);
                    sender.send((index, runner.fmt_value(line))).unwrap();
                } else {
                    sender.send((index, fmt_defvalue.clone())).unwrap();
                }
                thread::sleep(Duration::from_millis(25));
            }
        }),
    }
}

fn main() -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    let loggers = create_loggers();
    let mut values = vec![String::new(); loggers.len()];

    let handles: Vec<thread::JoinHandle<()>> = loggers
        .into_iter()
        .enumerate()
        .map(|(i, t)| spawn_thread(i, t, tx.clone()))
        .collect();

    let mut stdout = io::stdout();
    while let Ok((index, string)) = rx.recv() {
        values[index] = string;
        let value = values.join("");
        writeln!(stdout, "{}", value)?;
        stdout.flush()?;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
