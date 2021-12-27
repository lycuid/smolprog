# smolprog
**S**ystem **M**ontitoring and **O**rdered **L**ogging **PROG**ram.

This is pretty much similar to another, very well known, open source project called [**`slstatus`**](https://tools.suckless.org/slstatus/).  
This program runs different procedures (***logger***) in seperate threads and outputs the values in an ordered manner to ***stdout***.  
Due to the fact that this is multithreaded, logging can be done at ***indepedent intervals***.  
There are several default loggers provided, but are behind cargo's impressive 'feature flags', which prevents them from **being compiled**, if turned off.  
This project was developed to solve my personal and very specific problems (without open source in mind). But, if you can write Rust and understand the
given instructions, then feel free to use it in any way you want.  
To create your own custom ***logger***, follow [these](#create-custom-logger) steps.

## Use Case:
The fact that logging is done in the simplest possible manner (to stdout), this can be used in any number of ways.  
I personally use this to set the `WM_NAME` attribute of the root X11 window, which is then read and displayed by my statusbar: [xdbar](https://github.com/lycuid/xdbar/) (similar to [dwm's](https://dwm.suckless.org/) statusbar).  
Example:
```sh
smolprog | xargs -i xsetroot -name {}
```

## Build Requirements
  - [rust and cargo](https://www.rust-lang.org/)
  - [GNU make](https://www.gnu.org/software/make/) (optional)

## Build
Check out [Official Reference Manual](https://doc.rust-lang.org/cargo/reference/features.html) for compiling a ***cargo*** project with ***features***.  
Or ***&tldr;*** Build with:  
default features enabled.
```sh
cargo build --release
```
no features enabled.
```
cargo build --release --no-default-features
```
all features enabled.
```
cargo build --release --all-features
```
specific features enabled.
```
cargo build --release --features="date,cpu,memory"
```
***NOTE***: features can be found in `Cargo.toml` under `[features]`.

## Install
After building the project, run the following command to install.
```sh
sudo make install
```
if ***gnu make*** is not installed, then just try your best running the commands in the `Makefile` under `install`, manually.

## Create custom logger
Create a rust module in `src/loggers`
```diff
  src/
  ├── loggers/
  │   ├── some_logger.rs
+ |   └── my_logger.rs
```
Create an instance of a `Logger` variant.
```rust
// src/loggers/my_logger.rs

pub fn create_my_logger() -> Logger {
  Logger::ValueLogger { ... }
}
```
Include your module in `src/loggers.rs` file, and the your logger instance to the list of other instances in `create_loggers` function.
```rust
// src/loggers.rs
#[cfg(feature = "my_logger")]
mod my_logger;

pub fn create_loggers() -> Vec<Logger> {
  vec![
    ...
    #[cfg(feature = "my_logger")]
    my_logger::create_my_logger(),
    ...
  ]
}
```
Add the ***logger*** as ***feature*** in `Cargo.toml`.
```diff
  [features]
  ...
+ my_logger = []
```
