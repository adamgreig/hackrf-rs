# hackrf-rs
Rust bindings for libhackrf by Adam Greig, licensed under the MIT license.

Current status: alpha.

All the useful libhackrf functions are exposed (but not the ones to read/write
the MAX2837, the Si5351C, the RFFC5071, the SPI flash or the CPLD). That means
you can set the radio's parameters and send and receive data, but can't reflash
it through Rust.

As far as possible things are as safe as they're likely to be. The callback
system is inspired by Tomas Sedovic, and lets you pass a closure in and also
figures out all the memory magic so you don't have to write unsafe or C code
(see
http://www.aimlesslygoingforward.com/2014/09/18/safe-rust-callback-bindings/ ).

For some reason you cannot receive, stop receiving, then begin transmitting
without closing and re-opening the device in between. This behaviour is nothing
to do with Rust (a super simple C sketch demonstrates the same effect) so if
you have any ideas please shout.

`demo.rs` contains a very simple example that doesn't do anything interesting
with the radio data yet.

This is probably not the world's most idiosyncratic rust; please point out
anything that could be nicer. This probably isn't packaged or documented in the
best possible way either; likewise I'm open to suggestions.
