#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod llvm;
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    let path_components = &[];
    let path_descendants = &["llvm"];
    ::cxx_memory_abi::CxxAbiArtifactInfo::write_module_for_dir(path_components, path_descendants)
}
pub fn process_artifacts() -> ::cxx_memory_abi::BoxResult<()> {
    self::write_module()?;
    self::llvm::write_module()?;
    self::llvm::adt::write_module()?;
    self::llvm::adt::hash_code::write_module()?;
    self::llvm::adt::string_ref::write_module()?;
    self::llvm::adt::triple::write_module()?;
    self::llvm::adt::twine::write_module()?;
    Ok(())
}
