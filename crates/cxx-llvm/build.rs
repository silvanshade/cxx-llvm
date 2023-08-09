use cxx_llvm_build_common::prelude::*;
use std::path::{Path, PathBuf};

fn process_cxx(out_dir: &Path) -> BoxResult<()> {
    let cargo_pkg_name = "cxx-llvm";
    let dirs = cxx_llvm_build::Dirs::new(cargo_pkg_name)?;
    let rust_source_files = &[
        out_dir.join("src/auto/llvm/adt/hash_code.rs"),
        out_dir.join("src/auto/llvm/adt/string_ref.rs"),
        out_dir.join("src/auto/llvm/adt/triple.rs"),
        out_dir.join("src/auto/llvm/adt/twine.rs"),
        PathBuf::from("src/gen/llvm/adt/hash_code.rs"),
        PathBuf::from("src/gen/llvm/adt/string_ref.rs"),
        PathBuf::from("src/gen/llvm/adt/triple.rs"),
        PathBuf::from("src/gen/llvm/adt/twine.rs"),
    ];
    let files: &[&str] = &[];
    let output = "cxx-llvm";
    cxx_llvm_build::cxx_build(&dirs, rust_source_files, files, output)?;
    Ok(())
}

fn main() -> BoxResult<()> {
    println!("cargo:rerun-if-changed=src/gen");
    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir = PathBuf::from(out_dir);
    cxx_llvm_auto::auto::process_artifacts(&out_dir)?;
    process_cxx(&out_dir)?;
    Ok(())
}
