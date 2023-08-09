use cxx_llvm_build_common::prelude::*;
use std::path::{Path, PathBuf};

pub fn project_dir() -> BoxResult<PathBuf> {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let project_dir = std::path::PathBuf::from(&cargo_manifest_dir);
    Ok(project_dir)
}

fn process_cxx(out_dir: &Path) -> BoxResult<()> {
    let cargo_pkg_name = "cxx-llvm-auto";
    let llvm_dirs = cxx_llvm_build::Dirs::new(cargo_pkg_name)?;
    let includes = &[
        llvm_dirs.llvm_project.join("llvm/include"),
        llvm_dirs.llvm_cmake_build.join("include"),
    ];
    cxx_build::CFG
        .exported_header_dirs
        .extend(includes.iter().map(PathBuf::as_path));
    let rust_source_files = &[
        out_dir.join("src/auto/llvm/adt/hash_code.rs"),
        out_dir.join("src/auto/llvm/adt/string_ref.rs"),
        out_dir.join("src/auto/llvm/adt/triple.rs"),
        out_dir.join("src/auto/llvm/adt/twine.rs"),
    ];
    let files: &[&str] = &[];
    let output = "cxx-llvm-auto";
    cxx_llvm_build::cxx_build(&llvm_dirs, rust_source_files, files, output)?;
    Ok(())
}

fn main() -> BoxResult<()> {
    println!("cargo:rerun-if-changed=auto");
    println!("cargo:rerun-if-changed=cxx");
    let project_dir = project_dir()?;
    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir = PathBuf::from(out_dir);
    let cfg_dir = project_dir.join("auto");
    cxx_auto::process_artifacts(&project_dir, &out_dir, &cfg_dir)?;
    process_cxx(&out_dir)?;
    Ok(())
}
