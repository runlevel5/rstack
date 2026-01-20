use std::env;

fn main() {
    // Declare custom cfg flags for check-cfg
    println!("cargo::rustc-check-cfg=cfg(pre12)");
    println!("cargo::rustc-check-cfg=cfg(pre16)");

    let version = env::var("DEP_UNWIND_VERSION").unwrap();
    let mut it = version.split(&['.', '-'][..]);
    let major = it.next().unwrap().parse::<u32>().unwrap();
    let minor = it.next().unwrap().parse::<u32>().unwrap();
    if major < 1 || (major == 1 && minor < 2) {
        println!("cargo:rustc-cfg=pre12");
    }
    if major < 1 || (major == 1 && minor < 6) {
        println!("cargo:rustc-cfg=pre16");
    }
}
