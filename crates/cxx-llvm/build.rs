type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type BoxResult<T> = Result<T, BoxError>;

fn process_cxx() -> BoxResult<()> {
    let dirs = cxx_llvm_build::Dirs::new()?;
    let rust_source_files: &[&str] = &[
        "src/abi/llvm/adt/hash_code.rs",
        "src/abi/llvm/adt/string_ref.rs",
        "src/abi/llvm/adt/triple.rs",
        "src/abi/llvm/adt/twine.rs",
        "src/gen/llvm/adt/hash_code.rs",
        "src/gen/llvm/adt/string_ref.rs",
        "src/gen/llvm/adt/triple.rs",
        "src/gen/llvm/adt/twine.rs",
    ];
    let files: &[&str] = &[];
    let output = "cxx-clang";
    cxx_llvm_build::cxx_build(&dirs, rust_source_files, files, output)?;
    Ok(())
}

fn main() -> BoxResult<()> {
    println!("cargo:rerun-if-changed=src/gen");
    println!("cargo:rerun-if-changed=../cxx-llvm-abi");
    cxx_llvm_abi::abi::process_artifacts()?;
    process_cxx()?;
    Ok(())
}
