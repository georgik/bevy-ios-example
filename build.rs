use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    if target.contains("ios") {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=UIKit");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
        println!("cargo:rustc-link-lib=framework=AVFoundation");

        // Add iOS specific configurations
        println!("cargo:rustc-env=BEVY_MOBILE_PLATFORM=ios");

        // If this is an iOS simulator build, prepare app bundle for deployment
        if target.contains("ios-sim") {
            // Only run post-build steps if we're building (not just linking)
            if env::var("CARGO_CFG_TARGET_FEATURE").is_ok() {
                setup_ios_simulator_deployment(&target, &profile);
            }
        }
    }
}

fn setup_ios_simulator_deployment(target: &str, profile: &str) {
    println!("cargo:warning=Setting up iOS Simulator app bundle...");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir =
        env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| format!("{}/target", manifest_dir));

    let binary_name = env::var("CARGO_PKG_NAME").unwrap();
    let app_bundle_name = "RustApp.app";

    let binary_path = format!("{}/{}/{}/{}", target_dir, target, profile, binary_name);
    let app_bundle_path = format!("{}/{}", manifest_dir, app_bundle_name);
    let app_binary_path = format!("{}/rust-ios-test", app_bundle_path);

    // Create app bundle directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&app_bundle_path) {
        println!("cargo:warning=Failed to create app bundle directory: {}", e);
        return;
    }

    // Create Info.plist if it doesn't exist
    let info_plist_path = format!("{}/Info.plist", app_bundle_path);
    if !Path::new(&info_plist_path).exists() {
        let info_plist_content = create_info_plist_content();
        if let Err(e) = fs::write(&info_plist_path, info_plist_content) {
            println!("cargo:warning=Failed to create Info.plist: {}", e);
            return;
        }
        println!("cargo:warning=Created Info.plist at {}", info_plist_path);
    }

    // Wait for the binary to be built, then copy it
    if Path::new(&binary_path).exists() {
        if let Err(e) = fs::copy(&binary_path, &app_binary_path) {
            println!("cargo:warning=Failed to copy binary to app bundle: {}", e);
            return;
        }
        println!(
            "cargo:warning=Copied binary to app bundle: {}",
            app_binary_path
        );

        // Make binary executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(&app_binary_path) {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o755);
                let _ = fs::set_permissions(&app_binary_path, permissions);
            }
        }

        // Try to deploy to iOS Simulator automatically
        deploy_to_simulator(&app_bundle_path);
    } else {
        println!(
            "cargo:warning=Binary not found at {}, skipping app bundle setup",
            binary_path
        );
    }
}

fn deploy_to_simulator(app_bundle_path: &str) {
    println!("cargo:warning=Attempting to deploy to iOS Simulator...");

    // Check if any simulators are booted
    let list_output = Command::new("xcrun")
        .args(["simctl", "list", "devices", "--json"])
        .output();

    if let Ok(output) = list_output {
        if output.status.success() {
            let devices_json = String::from_utf8_lossy(&output.stdout);

            // Simple check for booted simulator (not parsing full JSON)
            if devices_json.contains("\"state\" : \"Booted\"") {
                println!("cargo:warning=Found booted simulator, installing app...");

                // Install the app to the booted simulator
                let install_result = Command::new("xcrun")
                    .args(["simctl", "install", "booted", app_bundle_path])
                    .output();

                match install_result {
                    Ok(output) if output.status.success() => {
                        println!("cargo:warning=✅ Successfully installed app to simulator");

                        // Try to launch the app
                        let launch_result = Command::new("xcrun")
                            .args(["simctl", "launch", "booted", "com.example.rustiostest"])
                            .output();

                        match launch_result {
                            Ok(output) if output.status.success() => {
                                println!("cargo:warning=🚀 Successfully launched app in simulator");

                                // Open Simulator app if not already open
                                let _ = Command::new("open").args(["-a", "Simulator"]).output();

                                println!("cargo:warning=📱 Simulator should now be visible with your app running");
                            }
                            Ok(output) => {
                                let error = String::from_utf8_lossy(&output.stderr);
                                println!("cargo:warning=Failed to launch app: {}", error);
                            }
                            Err(e) => {
                                println!("cargo:warning=Failed to execute launch command: {}", e);
                            }
                        }
                    }
                    Ok(output) => {
                        let error = String::from_utf8_lossy(&output.stderr);
                        println!("cargo:warning=Failed to install app: {}", error);
                    }
                    Err(e) => {
                        println!("cargo:warning=Failed to execute install command: {}", e);
                    }
                }
            } else {
                println!("cargo:warning=No booted simulator found. Boot a simulator first:");
                println!("cargo:warning=  xcrun simctl boot \"iPhone 15 Pro\"");

                // Try to boot a default simulator
                let boot_result = Command::new("xcrun")
                    .args(["simctl", "boot", "iPhone 15 Pro"])
                    .output();

                if let Ok(output) = boot_result {
                    if output.status.success() {
                        println!("cargo:warning=✅ Booted iPhone 15 Pro simulator");

                        // Wait a moment for simulator to boot, then try install again
                        std::thread::sleep(std::time::Duration::from_secs(2));

                        let install_result = Command::new("xcrun")
                            .args(["simctl", "install", "booted", app_bundle_path])
                            .output();

                        if let Ok(install_output) = install_result {
                            if install_output.status.success() {
                                println!(
                                    "cargo:warning=✅ Installed app to newly booted simulator"
                                );

                                let _ = Command::new("xcrun")
                                    .args(["simctl", "launch", "booted", "com.example.rustiostest"])
                                    .output();

                                let _ = Command::new("open").args(["-a", "Simulator"]).output();

                                println!(
                                    "cargo:warning=🚀 App should be launching in the simulator"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

fn create_info_plist_content() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>rust-ios-test</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.rustiostest</string>
    <key>CFBundleName</key>
    <string>Rust iOS Test</string>
    <key>CFBundleDisplayName</key>
    <string>Rust GUI</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSupportedPlatforms</key>
    <array>
        <string>iPhoneOS</string>
        <string>iPhoneSimulator</string>
    </array>
    <key>UIDeviceFamily</key>
    <array>
        <integer>1</integer>
        <integer>2</integer>
    </array>
    <key>UISupportedInterfaceOrientations</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
        <string>UIInterfaceOrientationPortraitUpsideDown</string>
    </array>
    <key>UIRequiredDeviceCapabilities</key>
    <array>
        <string>arm64</string>
    </array>
    <key>MinimumOSVersion</key>
    <string>12.0</string>
    <key>UIApplicationSceneManifest</key>
    <dict>
        <key>UIApplicationSupportsMultipleScenes</key>
        <false/>
        <key>UISceneConfigurations</key>
        <dict>
            <key>UIWindowSceneSessionRoleApplication</key>
            <array>
                <dict>
                    <key>UISceneConfigurationName</key>
                    <string>Default Configuration</string>
                    <key>UISceneDelegateClassName</key>
                    <string>SceneDelegate</string>
                </dict>
            </array>
        </dict>
    </dict>
</dict>
</plist>
"#.to_string()
}
