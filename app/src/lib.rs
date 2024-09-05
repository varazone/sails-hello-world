#![no_std]

use sails_rs::prelude::*;

struct SailsHelloWorldService(());

#[sails_rs::service]
impl SailsHelloWorldService {
    pub fn new() -> Self {
        Self(())
    }

    // Service's method (command)
    pub fn do_something(&mut self) -> String {
        "Hello from SailsHelloWorld!".to_string()
    }
}

pub struct SailsHelloWorldProgram(());

#[sails_rs::program]
impl SailsHelloWorldProgram {
    // Program's constructor
    pub fn new() -> Self {
        Self(())
    }

    // Exposed service
    pub fn sails_hello_world(&self) -> SailsHelloWorldService {
        SailsHelloWorldService::new()
    }
}
