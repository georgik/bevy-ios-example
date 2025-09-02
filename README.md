# Rust iOS Bevy GUI Application

A cross-platform GUI application built with Rust and the Bevy game engine, designed to run on both macOS and iOS (simulator and device).

## Features

- Interactive GUI with multiple buttons and state management
- Real-time button state tracking and display
- Cross-platform compatibility (macOS native and iOS)
- Clean, emoji-free text rendering for maximum font compatibility
- Built with Rust and Bevy for high performance

## Prerequisites

- **macOS** (required for iOS development)
- **Rust** (latest stable version)
- **Xcode** and iOS SDK
- **iOS Simulator** (comes with Xcode)

### Installing Rust iOS Targets

Before building for iOS, you need to add the iOS compilation targets:

```bash
# For iOS Simulator (ARM64, required for Apple Silicon Macs)
rustup target add aarch64-apple-ios-sim

# For iOS Devices
rustup target add aarch64-apple-ios
```

## Project Structure

```
rust-ios-test/
├── src/
│   └── main.rs              # Main application code
├── RustApp.app/             # iOS app bundle
│   ├── Info.plist          # iOS app metadata
│   └── rust-ios-test       # Compiled iOS binary
├── Cargo.toml              # Rust project configuration
├── .cargo/
│   └── config.toml         # Cargo build configuration
└── README.md               # This file
```

## Building and Running

### 1. macOS Native

To build and run the application natively on macOS:

```bash
cargo run --target aarch64-apple-darwin
```

### 2. iOS Simulator

#### Build for iOS Simulator

```bash
cargo build --target aarch64-apple-ios-sim
```

#### Create iOS App Bundle

The iOS app bundle should already exist, but if you need to recreate it:

```bash
mkdir -p RustApp.app
```

Copy the compiled binary to the app bundle:

```bash
cp target/aarch64-apple-ios-sim/debug/rust_ios_hello RustApp.app/rust-ios-test
```

#### Install and Run on iOS Simulator

1. **List available simulators:**
   ```bash
   xcrun simctl list devices
   ```

2. **Boot a simulator** (if none are running):
   ```bash
   xcrun simctl boot "iPhone 15 Pro"
   ```

3. **Install the app:**
   ```bash
   xcrun simctl install booted RustApp.app
   ```

4. **Launch the app:**
   ```bash
   xcrun simctl launch booted com.example.rustiostest
   ```

5. **Open Simulator GUI:**
   ```bash
   open -a Simulator
   ```

### 3. iOS Device

#### Build for iOS Device

```bash
cargo build --target aarch64-apple-ios
```

**Note:** Deploying to a physical iOS device requires proper code signing and provisioning profiles set up through Xcode.

## Configuration Details

### Cargo Configuration

The project includes special Cargo configuration in `.cargo/config.toml` to:
- Suppress raw pointer lint warnings from Bevy dependencies
- Ensure clean builds across different targets

### iOS Bundle Configuration

The `RustApp.app/Info.plist` contains:
- Bundle identifier: `com.example.rustiostest`
- Support for both iPhone and iPad
- Multiple orientation support

### Bevy iOS Integration

This project demonstrates:
- Conditional compilation for iOS-specific features
- Modified `bevy_log` crate to disable `tracing-oslog` on simulator
- Proper window configuration for iOS

## Troubleshooting

### Common Issues

1. **Build fails with "tracing-oslog" error on simulator:**
   - This is resolved by our modified Bevy configuration that disables `tracing-oslog` for simulator builds

2. **App doesn't launch on simulator:**
   - Ensure the iOS target is added: `rustup target add aarch64-apple-ios-sim`
   - Verify the app bundle structure is correct
   - Check that the simulator is booted

3. **Font rendering issues:**
   - This project uses emoji-free text to ensure compatibility with Bevy's font system

### Useful Commands

- **Check active simulators:** `xcrun simctl list devices | grep Booted`
- **View simulator logs:** `xcrun simctl spawn booted log stream --predicate 'process == "rust-ios-test"'`
- **Uninstall app:** `xcrun simctl uninstall booted com.example.rustiostest`

## Development Notes

### Text Rendering

This project specifically avoids emoji characters in all UI text because:
- Bevy's default font may not support emoji rendering
- Ensures consistent appearance across all platforms
- Prevents font fallback issues on iOS

### Performance Considerations

- The app is built in debug mode by default for development
- For production, use `cargo build --release --target <target>`
- Bevy applications benefit significantly from release mode optimizations

## Architecture

The application demonstrates:
- **Entity Component System (ECS)** architecture via Bevy
- **Cross-platform windowing** with proper iOS integration
- **State management** for interactive UI elements
- **Event handling** for button interactions

## Contributing

When contributing to this project:
1. Ensure all text remains emoji-free for font compatibility
2. Test on both macOS native and iOS simulator
3. Follow Rust and Bevy best practices
4. Update this README if adding new features or changing build processes

## License

This project is provided as-is for educational and development purposes.
