#![allow(non_snake_case)]

use std::path::Path;

fn main() {
    let wrapperPath = Path::new("embree/include/embree4/rtcore.h");
    let bindingsOutput = "src/bindings_embree.rs";
    let linkLibraryName = "embree4";

    // Rebuild on file change
    println!("cargo::rerun-if-changed={}", wrapperPath.display());

    // Link with system-wide embree lib
    println!("cargo:rustc-link-lib={}", linkLibraryName);

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header(format!("{}", wrapperPath.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("rtc.*")
        .allowlist_type("RTC.*")
        .allowlist_var("RTC.*")
        .rustified_enum("RTCFormat")
        .rustified_enum("RTCBuildQuality")
        .rustified_enum("RTCDeviceProperty")
        .rustified_enum("RTCError")
        .rustified_enum("RTCBufferType")
        .rustified_enum("RTCGeometryType")
        .rustified_enum("RTCSubdivisionMode")
        .bitfield_enum("RTC.*Flags")
        .generate()
        .expect("Unable to generate bindings");

    // Replace binding strings
    let bindingsSource = bindings
        .to_string()
        .replace("RTC_FORMAT_", "")
        .replace("RTC_BUILD_QUALITY_", "")
        .replace("RTC_RAY_QUERY_FLAG_", "")
        .replace("RTC_DEVICE_PROPERTY_", "")
        .replace("RTC_ERROR_", "")
        .replace("RTC_BUFFER_TYPE_", "")
        .replace("RTC_GEOMETRY_TYPE_", "")
        .replace("RTC_SUBDIVISION_MODE_", "")
        .replace("RTC_CURVE_FLAG_", "")
        .replace("RTC_SCENE_FLAG_", "")
        .replace("RTC_BUILD_FLAG_", "")
        .replace("RTC_FORMAT_", "")
        .replace(
            "pub type size_t = ::std::os::raw::c_ulong",
            "pub type size_t = usize",
        )
        .replace(
            "pub type __ssize_t = ::std::os::raw::c_long",
            "pub type __ssize_t = isize",
        )
        .replace(": ::std::os::raw::c_uint", ": u32");

    std::fs::write(bindingsOutput, bindingsSource)
        .expect("Could not write bindings to output path");
}
