#[derive(Clone, Debug, Default)]
pub struct LocationSnapshot {
    pub permission: String,
    pub coordinates: String,
    pub details: String,
    pub hint: String,
}

#[cfg(target_os = "ios")]
mod imp {
    use super::LocationSnapshot;
    use objc2::rc::Retained;
    use objc2_core_location::{
        CLAuthorizationStatus, CLLocationManager, kCLDistanceFilterNone, kCLLocationAccuracyBest,
    };

    pub struct LocationService {
        manager: Retained<CLLocationManager>,
        tracking: bool,
    }

    impl Default for LocationService {
        fn default() -> Self {
            Self::new()
        }
    }

    impl LocationService {
        pub fn new() -> Self {
            let manager = unsafe { CLLocationManager::new() };
            unsafe {
                manager.setDesiredAccuracy(kCLLocationAccuracyBest);
                manager.setDistanceFilter(kCLDistanceFilterNone);
                manager.setPausesLocationUpdatesAutomatically(false);
            }
            Self {
                manager,
                tracking: false,
            }
        }

        pub fn is_tracking(&self) -> bool {
            self.tracking
        }

        pub fn start(&mut self) -> LocationSnapshot {
            self.tracking = true;
            self.snapshot("Location updates requested.")
        }

        pub fn refresh(&mut self) -> LocationSnapshot {
            self.snapshot("Refreshing current location...")
        }

        pub fn stop(&mut self) -> LocationSnapshot {
            self.tracking = false;
            unsafe {
                self.manager.stopUpdatingLocation();
            }
            self.snapshot("Location updates stopped.")
        }

        pub fn poll(&mut self) -> LocationSnapshot {
            self.snapshot("Waiting for a simulated location fix...")
        }

        fn snapshot(&mut self, pending: &str) -> LocationSnapshot {
            let services_enabled = unsafe { CLLocationManager::locationServicesEnabled_class() };
            let status = unsafe { self.manager.authorizationStatus() };

            if self.tracking {
                match status {
                    CLAuthorizationStatus::NotDetermined => unsafe {
                        self.manager.requestWhenInUseAuthorization();
                    },
                    CLAuthorizationStatus::AuthorizedWhenInUse
                    | CLAuthorizationStatus::AuthorizedAlways => unsafe {
                        self.manager.startUpdatingLocation();
                    },
                    _ => {}
                }
            }

            if !services_enabled {
                return LocationSnapshot {
                    permission: "Permission: location services disabled".to_string(),
                    coordinates: "Coordinates: unavailable".to_string(),
                    details: "Details: enable Location Services for this simulator or device."
                        .to_string(),
                    hint: "Hint: open iOS Settings and re-enable system location services."
                        .to_string(),
                };
            }

            let permission = match status {
                CLAuthorizationStatus::NotDetermined => "Permission: waiting for iOS prompt",
                CLAuthorizationStatus::Restricted => "Permission: restricted by system",
                CLAuthorizationStatus::Denied => "Permission: denied in Settings",
                CLAuthorizationStatus::AuthorizedAlways => "Permission: authorized always",
                CLAuthorizationStatus::AuthorizedWhenInUse => "Permission: authorized when in use",
                _ => "Permission: unknown",
            }
            .to_string();

            if let Some(location) = unsafe { self.manager.location() } {
                let coordinate = unsafe { location.coordinate() };
                let horizontal_accuracy = unsafe { location.horizontalAccuracy() };
                let altitude = unsafe { location.altitude() };
                let speed = unsafe { location.speed() };

                let details = format!(
                    "Details: accuracy ±{horizontal_accuracy:.1}m | altitude {altitude:.1}m | speed {speed:.1}m/s"
                );
                let hint = if self.tracking {
                    "Hint: change Simulator > Features > Location to test movement.".to_string()
                } else {
                    "Hint: tap Start to keep polling for new simulated coordinates.".to_string()
                };

                return LocationSnapshot {
                    permission,
                    coordinates: format!(
                        "Coordinates: lat {:.5}, lon {:.5}",
                        coordinate.latitude, coordinate.longitude
                    ),
                    details,
                    hint,
                };
            }

            let hint = match status {
                CLAuthorizationStatus::NotDetermined => {
                    "Hint: accept the first location permission prompt on iOS.".to_string()
                }
                CLAuthorizationStatus::Denied | CLAuthorizationStatus::Restricted => {
                    "Hint: re-enable location access in iOS Settings for this app.".to_string()
                }
                _ if self.tracking => {
                    "Hint: pick a simulator route from Features > Location to generate a fix."
                        .to_string()
                }
                _ => "Hint: tap Start to begin requesting location updates.".to_string(),
            };

            LocationSnapshot {
                permission,
                coordinates: "Coordinates: unavailable".to_string(),
                details: format!("Details: {pending}"),
                hint,
            }
        }
    }
}

#[cfg(not(target_os = "ios"))]
mod imp {
    use super::LocationSnapshot;

    #[derive(Default)]
    pub struct LocationService {
        tracking: bool,
    }

    impl LocationService {
        pub fn is_tracking(&self) -> bool {
            self.tracking
        }

        pub fn start(&mut self) -> LocationSnapshot {
            self.tracking = true;
            self.snapshot("Desktop fallback active.")
        }

        pub fn refresh(&mut self) -> LocationSnapshot {
            self.snapshot("Desktop fallback active.")
        }

        pub fn stop(&mut self) -> LocationSnapshot {
            self.tracking = false;
            self.snapshot("Desktop fallback active.")
        }

        pub fn poll(&mut self) -> LocationSnapshot {
            self.snapshot("Desktop fallback active.")
        }

        fn snapshot(&self, details: &str) -> LocationSnapshot {
            LocationSnapshot {
                permission: "Permission: iOS bridge only".to_string(),
                coordinates: "Coordinates: unavailable on desktop preview".to_string(),
                details: format!("Details: {details}"),
                hint: "Hint: run ./run-ios.sh and test this page inside iOS Simulator.".to_string(),
            }
        }
    }
}

pub use imp::LocationService;
