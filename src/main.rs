mod blocks;
#[cfg(feature = "x11")]
mod xlib;
#[cfg(feature = "x11")]
use std::{ffi::CString, ptr};

use blocks::{createblks, Block};
use std::{
    fs,
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};

fn spawn_thread(
    index: usize,
    blk: &Block,
    sender: mpsc::Sender<(usize, String)>,
) -> thread::JoinHandle<()> {
    match &blk {
        Block::Value {
            default_value,
            interval_ms,
            create_runner,
        } => {
            let def = default_value.to_string();
            let delay = interval_ms.clone();
            let create_runner_func = Arc::clone(&create_runner);

            thread::spawn(move || {
                let mut runner = create_runner_func();
                sender.send((index, runner.fmt_value(def.clone()))).unwrap();

                loop {
                    let value = runner.get_value().unwrap_or(def.clone());
                    sender.send((index, runner.fmt_value(value))).unwrap();
                    thread::sleep(Duration::from_millis(delay));
                }
            })
        }

        Block::Fifo {
            default_value,
            fifopath,
            create_runner,
        } => {
            let def = default_value.to_string();
            let fifo = fifopath.clone();
            let create_runner_func = Arc::clone(&create_runner);

            thread::spawn(move || {
                let mut runner = create_runner_func();
                let defvalue = runner.fmt_value(def);
                sender.send((index, defvalue.clone())).unwrap();

                loop {
                    if let Some(ref fpath) = fifo {
                        fs::read_to_string(fpath)
                            .and_then(|data| {
                                let value =
                                    data.lines().next().unwrap().to_string();
                                Ok(sender
                                    .send((index, runner.fmt_value(value)))
                                    .unwrap())
                            })
                            .unwrap();
                    } else {
                        sender.send((index, defvalue.clone())).unwrap();
                    }
                    thread::sleep(Duration::from_millis(25));
                }
            })
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let blks = createblks();
    let mut values = vec![String::new(); blks.len()];

    let handles: Vec<thread::JoinHandle<()>> = blks
        .into_iter()
        .enumerate()
        .map(|(i, t)| spawn_thread(i, &t, tx.clone()))
        .collect();

    #[cfg(feature = "x11")]
    let (dpy, root) = unsafe {
        let dpy = xlib::XOpenDisplay(ptr::null());
        (dpy, xlib::XDefaultRootWindow(dpy))
    };

    while let Ok((index, string)) = rx.recv() {
        values[index] = string;
        let value = values.join("");

        #[cfg(feature = "x11")]
        unsafe {
            let wm_name = CString::new(value.clone()).unwrap();
            xlib::XStoreName(dpy, root, wm_name.as_ptr());
            xlib::XFlush(dpy);
        };

        #[cfg(not(feature = "x11"))]
        println!("{}", value);
    }

    #[cfg(feature = "x11")]
    unsafe {
        xlib::XCloseDisplay(dpy)
    };

    for handle in handles {
        handle.join().unwrap();
    }
}
