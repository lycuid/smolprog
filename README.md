# smolprog
**S**ystem **M**ontitoring and **O**rdered **L**ogging **PROG**ram.

This is pretty much similar to another, very well known, open source project called [slstatus](https://tools.suckless.org/slstatus/).  
This program runs different procedures (***logger***) in seperate threads and outputs the values in an ordered manner to ***stdout***.  


## Installation
```sh
sudo make install
```
build using `go build`.

## Use Case:
I personally use this to set the `WM_NAME` attribute of the root X11 window, which is then read and displayed by my statusbar: [clubar](https://github.com/lycuid/clubar/) (similar to [dwm's](https://dwm.suckless.org/) statusbar).  
Example:
```sh
smolprog | xargs -i xsetroot -name {}
```
