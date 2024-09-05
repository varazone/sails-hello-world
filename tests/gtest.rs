use sails_rs::{calls::*, gtest::calls::*};

use sails_hello_world_client::traits::*;

const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn do_something_works() {
    let remoting = GTestRemoting::new(ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(sails_hello_world::WASM_BINARY);

    let program_factory = sails_hello_world_client::SailsHelloWorldFactory::new(remoting.clone());

    let program_id = program_factory
        .new() // Call program's constructor (see src/lib.rs:27)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = sails_hello_world_client::SailsHelloWorld::new(remoting.clone());

    let result = service_client
        .do_something() // Call service's method (see src/lib.rs:17)
        .send_recv(program_id)
        .await
        .unwrap();

    assert_eq!(result, "Hello from SailsHelloWorld!".to_string());
}
