extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut bindings = bindgen::Builder::default();
    let mut any_bindings = false;

    if env::var("CARGO_FEATURE_CUBA").is_ok() {
        println!("cargo:rustc-link-lib=cuba");
        bindings = bindings.header("cuba_wrapper.h");
        any_bindings = true;
    }

    if env::var("CARGO_FEATURE_GSL").is_ok() {
        //println!("cargo:rustc-link-lib=gsl");
        let gsl_cblas = if env::var("CARGO_FEATURE_NO_GSLCBLAS").is_ok(){
            "gsl"
        } else {
            //println!("cargo:rustc-link-lib=gslcblas");
            "gslcblas"
        };
        let gsl = pkg_config::Config::new()
            .arg(format!("--define-variable=GSL_CBLAS_LIB=-l{}",gsl_cblas))
            .probe("gsl")
            .unwrap();
        let include_str : Vec<String>= gsl.include_paths
            .iter()
            .map(|p| format!("-I{}", p.to_owned().to_str().unwrap()))
            .collect()
            //fold(String::new(),
            //      |mut i, p| { i.push_str(&format!("-I{} ", p.to_owned().to_str().unwrap())); i } )
            ;

        bindings = bindings.header("gsl_wrapper.h").clang_args(include_str.iter());
        any_bindings = true;
    }

    if any_bindings {
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings.blacklist_item("FP_NORMAL")
                .blacklist_item("FP_SUBNORMAL")
                .blacklist_item("FP_ZERO")
                .blacklist_item("FP_INFINITE")
                .blacklist_item("FP_NAN")
                .generate().unwrap()
                .write_to_file(out_path.join("integrand_bindings.rs"))
                .expect("Error writing bindings");
    }
}
