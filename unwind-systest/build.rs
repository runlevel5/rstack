use std::env;

fn main() {
    let mut cfg = ctest2::TestGenerator::new();

    let includedir = env::var_os("DEP_UNWIND_INCLUDEDIR").unwrap();
    cfg.include(includedir);

    if cfg!(feature = "ptrace") {
        cfg.cfg("feature", Some("ptrace"))
            .header("libunwind-ptrace.h");
    }

    let version = env::var("DEP_UNWIND_VERSION").unwrap();
    let mut it = version.split(".");
    let major = it.next().unwrap().parse::<u32>().unwrap();
    let mut minor = it.next().unwrap().parse::<u32>().unwrap();
    // the pkg-config version is messed up in old versions and reports e.g. 1.21 for 1.2.1!
    if it.next().is_none() {
        minor /= 10;
    }
    if major < 1 || (major == 1 && minor < 2) {
        cfg.cfg("pre12", None);
    }
    if major < 1 || (major == 1 && minor < 3) {
        println!("cargo:rustc-cfg=pre13");
    }
    if major < 1 || (major == 1 && minor < 4) {
        cfg.cfg("pre14", None);
    }
    if major < 1 || (major == 1 && minor < 6) {
        cfg.cfg("pre16", None);
    }
    let pre17 = major < 1 || (major == 1 && minor < 7);
    if pre17 {
        cfg.cfg("pre17", None);
    }
    let pre18 = major < 1 || (major == 1 && minor < 8);
    if pre18 {
        cfg.cfg("pre18", None);
    }

    cfg.header("libunwind.h")
        .type_name(|t, _, _| match t {
            "unw_sigcontext" => format!("struct {}", t),
            _ => t.to_string(),
        })
        .skip_signededness(|t| match t {
            "unw_tdep_fpreg_t" | "unw_tdep_context_t" | "unw_context_t" | "unw_addr_space_t" => {
                true
            }
            _ => false,
        })
        .field_name(|s, f| match (s, f) {
            ("unw_save_loc_t", "type_") => "type".to_string(),
            (_, f) => f.to_string(),
        })
        .skip_type(|s| match s {
            // https://github.com/rust-lang/libc/issues/1410
            "unw_tdep_context_t" | "unw_context_t" => true,
            _ => false,
        })
        .skip_struct(|s| match s {
            "unw_save_loc_t_u" => true,
            _ => false,
        })
        .skip_field(move |s, f| match (s, f) {
            // ptrauth_insn_mask was added in libunwind 1.7.0
            ("unw_accessors_t", "ptrauth_insn_mask") => pre17,
            // For non-x86_64 architectures, unused field was added in libunwind 1.8.0
            // x86_64 always had the unused field (as char, changed to uint8_t in 1.8)
            ("unw_tdep_save_loc_t", "unused") | ("unw_tdep_proc_info_t", "unused") => {
                let target = env::var("TARGET").unwrap();
                if target.contains("x86_64") {
                    false // x86_64 always has this field
                } else {
                    pre18 // other archs only have it in 1.8+
                }
            }
            _ => false,
        })
        .skip_field_type(|s, f| match (s, f) {
            ("unw_save_loc_t", "u") => true,
            _ => false,
        })
        // i686 and ppc64 ABI disagrees about how to handle ZST-by-value
        .skip_roundtrip(|t| {
            let target = env::var("TARGET").unwrap();
            (target.contains("i686") || target.contains("powerpc64"))
                && match t {
                    "unw_tdep_save_loc_t" | "unw_tdep_proc_info_t" => true,
                    _ => false,
                }
        })
        .generate("../unwind-sys/src/lib.rs", "all.rs");
}
