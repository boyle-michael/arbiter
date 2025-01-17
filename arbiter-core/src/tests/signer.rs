use std::sync::Arc;

use super::*;

#[test]
fn simulation_signer() {
    let params = EnvironmentParameters {
        block_rate: 1.0,
        seed: 1,
    };
    let environment = &mut Environment::new(TEST_ENV_LABEL, params);
    let client = Arc::new(RevmMiddleware::new(
        environment,
        Some(TEST_SIGNER_SEED_AND_LABEL.to_string()),
    ));
    assert_eq!(
        client.default_sender().unwrap(),
        Address::from_str("0x2efdc9eecfee3a776209fcb8e9a83a6b221d74f5").unwrap()
    );
}

#[test]
fn multiple_signer_addresses() {
    let params = EnvironmentParameters {
        block_rate: 1.0,
        seed: 1,
    };
    let environment = &mut Environment::new(TEST_ENV_LABEL, params);
    let client_1 = Arc::new(RevmMiddleware::new(environment, Some("0".to_string())));
    let client_2 = Arc::new(RevmMiddleware::new(environment, Some("1".to_string())));
    assert_ne!(client_1.default_sender(), client_2.default_sender());
}

#[test]
fn signer_collision() {
    let params = EnvironmentParameters {
        block_rate: 1.0,
        seed: 1,
    };
    let environment = &mut Environment::new(TEST_ENV_LABEL, params);
    let client_1 = Arc::new(RevmMiddleware::new(environment, Some("0".to_string())));
    let client_2 = Arc::new(RevmMiddleware::new(environment, Some("0".to_string())));
    assert_eq!(client_1.default_sender(), client_2.default_sender());
}
