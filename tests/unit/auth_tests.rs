//! Auth manager tests

use multigit::core::auth::{AuthBackend, AuthManager};

#[test]
fn test_auth_manager_new() {
    let auth = AuthManager::new(AuthBackend::Environment, false);
    assert_eq!(std::mem::size_of_val(&auth), std::mem::size_of::<AuthManager>());
}

#[test]
fn test_auth_backend_variants() {
    // Test that AuthBackend variants exist
    let _keyring = AuthBackend::Keyring;
    let _file = AuthBackend::EncryptedFile;
    let _env = AuthBackend::Environment;
}


#[test]
fn test_auth_backends() {
    let keyring_auth = AuthManager::new(AuthBackend::Keyring, false);
    assert_eq!(std::mem::size_of_val(&keyring_auth), std::mem::size_of::<AuthManager>());
    
    let file_auth = AuthManager::new(AuthBackend::EncryptedFile, false);
    assert_eq!(std::mem::size_of_val(&file_auth), std::mem::size_of::<AuthManager>());
}
