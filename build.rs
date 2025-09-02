use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

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
    }
}
