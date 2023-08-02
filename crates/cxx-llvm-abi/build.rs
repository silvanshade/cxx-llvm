use cxx_llvm_build_common::prelude::*;
use std::path::PathBuf;

pub fn project_dir() -> BoxResult<std::path::PathBuf> {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let project_dir = std::path::PathBuf::from(&cargo_manifest_dir);
    Ok(project_dir)
}

fn process_cxx() -> BoxResult<()> {
    let cargo_pkg_name = "cxx-llvm-abi";
    let llvm_dirs = cxx_llvm_build::Dirs::new(cargo_pkg_name)?;
    let includes = &[
        llvm_dirs.llvm_project.join("llvm/include"),
        llvm_dirs.llvm_cmake_build.join("include"),
    ];
    cxx_build::CFG
        .exported_header_dirs
        .extend(includes.iter().map(PathBuf::as_path));
    let rust_source_files: &[&str] = &[
        "src/abi/llvm/adt/hash_code.rs",
        "src/abi/llvm/adt/string_ref.rs",
        "src/abi/llvm/adt/triple.rs",
        "src/abi/llvm/adt/twine.rs",
    ];
    let files: &[&str] = &[];
    let output = "cxx-llvm-abi";
    cxx_llvm_build::cxx_build(&llvm_dirs, rust_source_files, files, output)?;
    Ok(())
}

fn main() -> BoxResult<()> {
    println!("cargo:rerun-if-changed=abi");
    println!("cargo:rerun-if-changed=cxx");
    let project_dir = project_dir()?;
    let abi_dir = project_dir.join("abi");
    let abi_crate_src_dir = project_dir.join("src");
    cxx_memory_abi::process_artifacts(&abi_dir, &abi_crate_src_dir)?;
    process_cxx()?;
    Ok(())
}
