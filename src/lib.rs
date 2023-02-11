mod err;
mod api;
mod config;
mod nss;

#[cfg(test)]
mod tests;

extern crate libc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate libnss;
