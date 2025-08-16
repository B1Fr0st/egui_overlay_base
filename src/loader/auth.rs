use regex::Regex;
use std::sync::mpsc;
use std::thread;
use crate::loader::app::*;

impl crate::loader::app::MyApp {
    pub fn verify_license_async(&mut self) {
        self.failed_reason = String::new();
        
        // Create channel for communication
        let (tx, rx) = mpsc::channel();
        self.license_receiver = Some(rx);
        
        let license = self.license.clone();
        
        // Spawn background thread for license verification
        thread::spawn(move || {
            let mut keyauthapp = crate::loader::keyauth::new();
            
            let result = match keyauthapp.init(None) {
                Ok(_) => {
                    match keyauthapp.license(license.clone(), None) {
                        Ok(_) => {
                            keyauthapp.log(format!("License {} successfully logged in!", license), None);
                            LicenseResult::Success
                        }
                        Err(e) => {
                            keyauthapp.log(format!("License {} failed to log in: {}", license, e), None);
                            LicenseResult::Error(e)
                        }
                    }
                }
                Err(e) => LicenseResult::Error(format!("Failed to initialize KeyAuth: {}", e))
            };
            
            // Send result back to main thread
            let _ = tx.send(result);
        });
    }
    
    pub fn check_license_result(&mut self) {
        if let Some(ref receiver) = self.license_receiver {
            match receiver.try_recv() {
                Ok(result) => {
                    self.license_receiver = None;
                    
                    match result {
                        LicenseResult::Success => {
                            self.ui_state = UiState::Verified;
                        }
                        LicenseResult::Error(error) => {
                            self.failed_reason = error;
                            self.license = String::new();
                            self.ui_state = UiState::Error;
                        }
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {
                    // Still waiting for result
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    // Thread panicked or channel closed
                    self.license_receiver = None;
                    self.failed_reason = "Error: 0xA011".to_string();
                    self.license = String::new();
                    self.ui_state = UiState::Error;
                }
            }
        }
    }

    pub fn license_regex(&self) -> bool {
        let regex = Regex::new(r"(?m)KEYAUTH-[a-zA-Z0-9]{6}-[a-zA-Z0-9]{6}-[a-zA-Z0-9]{6}-[a-zA-Z0-9]{6}-[a-zA-Z0-9]{6}-[a-zA-Z0-9]{6}").unwrap();
        regex.is_match(&self.license)
    }
}