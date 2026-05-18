# gigu Demo

Minimal Makepad demo app for iOS and Android-oriented development.

## iOS simulator

Current booted simulator:

- `iPhone 16e`

Run the main demo with:

```bash
./scripts/run-ios.sh
```

Run a different bin, for example the map demo, with:

```bash
./scripts/run-ios.sh map_demo
```

Tail simulator logs in another terminal with:

```bash
./scripts/ios-log.sh
```

This main demo now includes:

- Home workflow page
- Camera page based on the official Makepad camera example
- Location page with an iOS CoreLocation bridge
- Debug page for app log verification

Useful iOS dev loop:

```bash
./scripts/run-ios.sh
./scripts/ios-log.sh
```

Equivalent raw Makepad command:

```bash
cargo makepad ios --org=dev.gigu --app=gigu run-sim -p gigu
```

Note: the installed `cargo-makepad` v0.4.0 produces an `Info.plist` without the
iOS privacy usage-description keys (`NSCameraUsageDescription`,
`NSLocationWhenInUseUsageDescription`, ...). The bare command above will build
and install fine but the app crashes immediately on launch with `SIGABRT` and a
TCC termination reason. `scripts/run-ios.sh` works around that by patching the
installed bundle's `Info.plist` and relaunching via `xcrun simctl launch`.

Notes:

- iOS Simulator usually has no real camera device, so the camera page may report no camera found.
- For simulated GPS, use Simulator.app > Features > Location.

## Android

The Android SDK download started successfully, but `cargo-makepad` failed while extracting the NDK on this machine.

Retry with:

```bash
cargo makepad android --abi=all install-toolchain
```

If that keeps failing, try:

```bash
cargo makepad android --abi=all --full-ndk install-toolchain
```
