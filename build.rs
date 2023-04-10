extern crate rustc_version;

use rustc_version::{version_meta, Channel};

fn main() {
    let version_info = match version_meta() {
        Ok(v) => v,
        Err(e) => panic!("could not identify rustc version. error: {:?}", e),
    };

    match version_info.channel {
        Channel::Beta | Channel::Stable  if version_info.semver.minor < 59 => {
            panic!("this crate is not supported on the stable, or beta versions");
        }
        _ => {}
    };

    // determine the kind of asm to use
    if version_info.semver.major > 1 {
        panic!("please update this crate with the breaking rustc 2.0 changes.")
    } else if version_info.semver.minor >= 59 {
        // nothing to do.  asm macro stabalized in version 1.59
    } else if version_info.semver.minor >= 46 {
        println!(r#"cargo:rustc-cfg=feature="LLVM_ASM""#);
    } else {
        println!(r#"cargo:rustc-cfg=feature="OLD_ASM""#);
    }
}
