mod loggers;

use loggers::{create_loggers, Logger};
use std::{
    fs,
    io::{self, Write},
    sync::mpsc,
    thread,
    time::Duration,
};

type ThreadPayload = (usize, Logger, mpsc::Sender<(usize, String)>);

fn spawn_thread(payload: ThreadPayload) -> thread::JoinHandle<()> {
    let (id, logger, sender) = payload;
    match logger {
        Logger::ValueLogger {
            interval_ms,
            mut runner,
        } => thread::spawn(move || loop {
            sender.send((id, runner.get_value())).unwrap();
            thread::sleep(Duration::from_millis(interval_ms));
        }),

        Logger::FifoLogger {
            default_value,
            fifopath,
            mut runner,
        } => thread::spawn(move || {
            let fmt_defvalue = runner.fmt_value(&default_value);
            sender.send((id, fmt_defvalue.clone())).unwrap();

            loop {
                if let Ok(data) = fs::read_to_string(&fifopath) {
                    let line = data.lines().next().unwrap_or(&default_value);
                    sender.send((id, runner.fmt_value(line))).unwrap();
                } else {
                    sender.send((id, fmt_defvalue.clone())).unwrap();
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

    let threads: Vec<thread::JoinHandle<()>> = loggers
        .into_iter()
        .enumerate()
        .map(|(i, l)| spawn_thread((i, l, tx.clone())))
        .collect();

    let mut stdout = io::stdout();
    while let Ok((index, string)) = rx.recv() {
        if let Some(block) = values.get_mut(index) {
            if *block != string {
                *block = string;
                stdout.write(values.join("").as_bytes())?;
                stdout.write("\n".as_bytes())?;
                stdout.flush()?;
            }
        }
    }

    Ok(for thread in threads {
        thread.join().unwrap()
    })
}
