use cxx_llvm_build_common::prelude::*;

fn process_cxx() -> BoxResult<()> {
    let cargo_pkg_name = "cxx-llvm";
    let dirs = cxx_llvm_build::Dirs::new(cargo_pkg_name)?;
    let rust_source_files: &[&str] = &[
        "src/auto/llvm/adt/hash_code.rs",
        "src/auto/llvm/adt/string_ref.rs",
        "src/auto/llvm/adt/triple.rs",
        "src/auto/llvm/adt/twine.rs",
        "src/gen/llvm/adt/hash_code.rs",
        "src/gen/llvm/adt/string_ref.rs",
        "src/gen/llvm/adt/triple.rs",
        "src/gen/llvm/adt/twine.rs",
    ];
    let files: &[&str] = &[];
    let output = "cxx-llvm";
    cxx_llvm_build::cxx_build(&dirs, rust_source_files, files, output)?;
    Ok(())
}

fn main() -> BoxResult<()> {
    println!("cargo:rerun-if-changed=src/gen");
    println!("cargo:rerun-if-changed=../cxx-llvm-auto");
    cxx_llvm_auto::auto::process_artifacts()?;
    process_cxx()?;
    Ok(())
}
