#![doc = r" NOTE: This module is auto-generated and should not be edited."]
pub mod hash_code;
pub mod string_ref;
pub mod triple;
pub mod twine;
pub(crate) fn write_module() -> ::cxx_memory_abi::BoxResult<()> {
    let path_components = &["llvm", "adt"];
    let path_descendants = &["hash_code", "string_ref", "triple", "twine"];
    ::cxx_memory_abi::CxxAbiArtifactInfo::write_module_for_dir(path_components, path_descendants)
}
