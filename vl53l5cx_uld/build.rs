use std::fs;
use itertools::Itertools;

const FN: &str = "tmp/config.h";

fn main() {

    //---
    // Sanity checks - often some configs would be mutually dependent; see that they are used right
    //          #placeholder
    //
    #[cfg(not(all(feature = "nb_target_detected", feature = "target_status")))]
    {
        println!("cargo::warning={}", "Vendor docs: \"To ensure data consistency, [vendor]
                always recommends keeping the ‘number of targets detected’ and ‘target status’ enabled\".
                Your features are missing one or both or these - is this intentional?"
        );
    }

    //---
    // Create a C config header, based on the features from 'Cargo.toml'.
    {
        let mut defs: Vec<&str> = vec!();

        // Output-enabling features (in Rust, we have them enabling; in C they are disable flags). Same thing.
        #[cfg(not(feature = "ambient_per_spad"))]
        defs.push("VL53L5CX_DISABLE_AMBIENT_PER_SPAD");
        #[cfg(not(feature = "nb_spads_enabled"))]
        defs.push("VL53L5CX_DISABLE_NB_SPADS_ENABLED");
        #[cfg(not(feature = "nb_target_detected"))]
        defs.push("VL53L5CX_DISABLE_NB_TARGET_DETECTED");
        #[cfg(not(feature = "signal_per_spad"))]
        defs.push("VL53L5CX_DISABLE_SIGNAL_PER_SPAD");
        #[cfg(not(feature = "range_sigma_mm"))]
        defs.push("VL53L5CX_DISABLE_RANGE_SIGMA_MM");
        #[cfg(not(feature = "distance_mm"))]
        defs.push("VL53L5CX_DISABLE_DISTANCE_MM");
        #[cfg(not(feature = "target_status"))]
        defs.push("VL53L5CX_DISABLE_TARGET_STATUS");
        #[cfg(not(feature = "reflectance_percent"))]
        defs.push("VL53L5CX_DISABLE_REFLECTANCE_PERCENT");
        #[cfg(not(feature = "motion_indicator"))]
        defs.push("VL53L5CX_DISABLE_MOTION_INDICATOR");

        #[cfg(feature = "use_raw_format")]
        defs.push("VL53L5CX_USE_RAW_FORMAT");

        // Write the file. This way the last 'cargo build' state remains available, even if
        // 'make' were run manually (compared to passing individual defines to the 'Makefile');
        // also, it keeps the 'Makefile' simple.
        //
        let contents = defs.iter()
            .map(|s| format!("#define {s}"))
            .join("\n");

        fs::write(FN, contents)
            .expect("Unable to write a file");  // note: cannot pass 'FN' here; tbd.
    }

    // make stuff
    //
    let st = std::process::Command::new("make")
        .arg("tmp/vendor_uld.a")    // ULD C library
        .arg("src/uld_raw.rs")      // generate the ULD Rust bindings
        .output()
        .expect("could not spawn `make`")   // shown if 'make' not found on PATH
        .status;

    assert!(st.success(), "Running 'make' failed");    // shown if 'make' returns a non-zero

    // Link with 'tmp/vendor_uld.a'
    println!("cargo:rustc-link-search=tmp");
    println!("cargo:rustc-link-lib=vendor_uld");
}
