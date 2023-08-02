use cxx_llvm_build_common::prelude::*;
use normpath::PathExt;
use std::path::{Path, PathBuf};

fn locate_swift_project_path(cargo_manifest_dir: &Path) -> BoxResult<Option<PathBuf>> {
    let mut should_ignore = false;

    if let Some(swift_project) = get_path_from_env("SWIFT_PROJECT_PATH", &mut should_ignore)? {
        return Ok(Some(swift_project));
    }

    if should_ignore {
        return Ok(None);
    }

    let swift_project = PathBuf::from(cargo_manifest_dir).join("../../../swift-project");
    if swift_project.exists() {
        return Ok(Some(swift_project.normalize()?.into_path_buf()));
    }

    Ok(None)
}

fn locate_llvm_project_path(cargo_manifest_dir: &Path, swift_project: Option<&Path>) -> BoxResult<PathBuf> {
    let err =
        Err(r#"Failed to locate LLVM project path. Try setting the `LLVM_PROJECT_PATH` environment variable."#.into());

    let mut should_ignore = false;

    if let Some(llvm_project) = get_path_from_env("LLVM_PROJECT_PATH", &mut should_ignore)? {
        return Ok(llvm_project);
    }

    if should_ignore {
        return err;
    }

    if let Some(swift_project) = swift_project {
        let llvm_project = swift_project.join("llvm-project");
        if llvm_project.exists() {
            return Ok(llvm_project);
        }
    }

    let llvm_project = PathBuf::from(cargo_manifest_dir).join("../../../llvm-project");
    if llvm_project.exists() {
        return Ok(llvm_project.normalize()?.into_path_buf());
    }

    err
}

fn locate_llvm_cmake_build_path(swift_project: Option<&Path>, llvm_project: &Path) -> BoxResult<PathBuf> {
    let err = Err(
        r#"Failed to locate LLVM CMake build path. Try setting the `LLVM_CMAKE_BUILD_PATH` environment variable."#
            .into(),
    );

    let mut should_ignore = false;

    if let Some(llvm_cmake_build) = get_path_from_env("LLVM_CMAKE_BUILD_PATH", &mut should_ignore)? {
        return Ok(llvm_cmake_build);
    }

    if should_ignore {
        return err;
    }

    if let Some(swift_project) = swift_project {
        let llvm_cmake_build = swift_project.join(PathBuf::from_iter([
            "build",
            &NINJA_BUILD_DIR(),
            &format!("llvm-{CMAKE_BUILD_TARGET}"),
        ]));
        if llvm_cmake_build.exists() {
            return Ok(llvm_cmake_build);
        }
    }

    let llvm_cmake_build = llvm_project.join(PathBuf::from_iter([
        "build",
        &NINJA_BUILD_DIR(),
        &format!("llvm-{CMAKE_BUILD_TARGET}"),
    ]));
    if llvm_cmake_build.exists() {
        return Ok(llvm_cmake_build);
    }

    err
}

pub struct Dirs {
    pub swift_project: Option<PathBuf>,
    pub llvm_project: PathBuf,
    pub llvm_cmake_build: PathBuf,
}

impl Dirs {
    pub fn new(cargo_pkg_name: &str) -> BoxResult<Self> {
        let cargo_manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
        let swift_project = locate_swift_project_path(&cargo_manifest_dir)?;
        if let Some(swift_project) = swift_project.as_deref() {
            println!(
                "cargo:warning=[{cargo_pkg_name}]: Swift project path: \"{}\"",
                swift_project.display()
            );
        }
        let llvm_project = locate_llvm_project_path(&cargo_manifest_dir, swift_project.as_deref())?;
        println!(
            "cargo:warning=[{cargo_pkg_name}]: LLVM project path: \"{}\"",
            llvm_project.display()
        );

        let llvm_cmake_build = locate_llvm_cmake_build_path(swift_project.as_deref(), &llvm_project)?;
        println!(
            "cargo:warning=[{cargo_pkg_name}]: LLVM CMake build path: \"{}\"",
            llvm_cmake_build.display().to_string()
        );
        let dirs = Dirs {
            swift_project,
            llvm_project,
            llvm_cmake_build,
        };
        Ok(dirs)
    }
}

pub fn cxx_build(
    dirs: &Dirs,
    rust_source_files: impl IntoIterator<Item = impl AsRef<Path>>,
    files: impl IntoIterator<Item = impl AsRef<Path>>,
    output: &str,
) -> BoxResult<()> {
    rustc_link_search(dirs);
    cxx_build::bridges(rust_source_files)
        .llvm_common_compiler()
        .llvm_common_defines()
        .llvm_common_flags()
        .files(files)
        .try_compile(output)?;
    rustc_link_libs();
    Ok(())
}

fn rustc_link_search(dirs: &Dirs) {
    println!(
        "cargo:rustc-link-search={}",
        dirs.llvm_cmake_build.join("lib").display()
    );
}

fn rustc_link_libs() {
    link_llvm_libs();
}

fn link_llvm_libs() {
    println!("cargo:rustc-link-lib=static=LLVMX86Disassembler");
    println!("cargo:rustc-link-lib=static=LLVMARMDisassembler");
    println!("cargo:rustc-link-lib=static=LLVMAArch64Disassembler");
    println!("cargo:rustc-link-lib=static=LLVMPowerPCDisassembler");
    println!("cargo:rustc-link-lib=static=LLVMSystemZDisassembler");
    println!("cargo:rustc-link-lib=static=LLVMMipsDisassembler");
    println!("cargo:rustc-link-lib=static=LLVMCoverage");
    println!("cargo:rustc-link-lib=static=LLVMX86AsmParser");
    println!("cargo:rustc-link-lib=static=LLVMARMAsmParser");
    println!("cargo:rustc-link-lib=static=LLVMAArch64AsmParser");
    println!("cargo:rustc-link-lib=static=LLVMPowerPCAsmParser");
    println!("cargo:rustc-link-lib=static=LLVMSystemZAsmParser");
    println!("cargo:rustc-link-lib=static=LLVMMipsAsmParser");
    println!("cargo:rustc-link-lib=static=LLVMX86Info");
    println!("cargo:rustc-link-lib=static=LLVMARMInfo");
    println!("cargo:rustc-link-lib=static=LLVMAArch64Info");
    println!("cargo:rustc-link-lib=static=LLVMPowerPCInfo");
    println!("cargo:rustc-link-lib=static=LLVMSystemZInfo");
    println!("cargo:rustc-link-lib=static=LLVMMipsInfo");
    println!("cargo:rustc-link-lib=static=LLVMX86CodeGen");
    println!("cargo:rustc-link-lib=static=LLVMARMCodeGen");
    println!("cargo:rustc-link-lib=static=LLVMAArch64CodeGen");
    println!("cargo:rustc-link-lib=static=LLVMPowerPCCodeGen");
    println!("cargo:rustc-link-lib=static=LLVMSystemZCodeGen");
    println!("cargo:rustc-link-lib=static=LLVMMipsCodeGen");
    println!("cargo:rustc-link-lib=static=LLVMX86Desc");
    println!("cargo:rustc-link-lib=static=LLVMARMDesc");
    println!("cargo:rustc-link-lib=static=LLVMAArch64Desc");
    println!("cargo:rustc-link-lib=static=LLVMPowerPCDesc");
    println!("cargo:rustc-link-lib=static=LLVMSystemZDesc");
    println!("cargo:rustc-link-lib=static=LLVMMipsDesc");
    println!("cargo:rustc-link-lib=static=LLVMARMUtils");
    println!("cargo:rustc-link-lib=static=LLVMAArch64Utils");
    println!("cargo:rustc-link-lib=static=LLVMCFGuard");
    println!("cargo:rustc-link-lib=static=LLVMAsmPrinter");
    println!("cargo:rustc-link-lib=static=LLVMDebugInfoCodeView");
    println!("cargo:rustc-link-lib=static=LLVMGlobalISel");
    println!("cargo:rustc-link-lib=static=LLVMSelectionDAG");
    println!("cargo:rustc-link-lib=static=LLVMCodeGen");
    println!("cargo:rustc-link-lib=static=LLVMMCDisassembler");
    println!("cargo:rustc-link-lib=static=LLVMLTO");
    println!("cargo:rustc-link-lib=static=LLVMTarget");
    println!("cargo:rustc-link-lib=static=LLVMObjCARCOpts");
    println!("cargo:rustc-link-lib=static=LLVMScalarOpts");
    println!("cargo:rustc-link-lib=static=LLVMInstrumentation");
    println!("cargo:rustc-link-lib=static=LLVMipo");
    println!("cargo:rustc-link-lib=static=LLVMLinker");
    println!("cargo:rustc-link-lib=static=LLVMIRReader");
    println!("cargo:rustc-link-lib=static=LLVMAsmParser");
    println!("cargo:rustc-link-lib=static=LLVMScalarOpts");
    println!("cargo:rustc-link-lib=static=LLVMVectorize");
    println!("cargo:rustc-link-lib=static=LLVMAggressiveInstCombine");
    println!("cargo:rustc-link-lib=static=LLVMInstCombine");
    println!("cargo:rustc-link-lib=static=LLVMBitWriter");
    println!("cargo:rustc-link-lib=static=LLVMPasses");
    println!("cargo:rustc-link-lib=static=LLVMCoroutines");
    println!("cargo:rustc-link-lib=static=LLVMTransformUtils");
    println!("cargo:rustc-link-lib=static=LLVMAnalysis");
    println!("cargo:rustc-link-lib=static=LLVMProfileData");
    println!("cargo:rustc-link-lib=static=LLVMDebugInfoDWARF");
    println!("cargo:rustc-link-lib=static=LLVMObject");
    println!("cargo:rustc-link-lib=static=LLVMBitReader");
    println!("cargo:rustc-link-lib=static=LLVMMCParser");
    println!("cargo:rustc-link-lib=static=LLVMTextAPI");
    println!("cargo:rustc-link-lib=static=LLVMWindowsDriver");
    println!("cargo:rustc-link-lib=static=LLVMOption");
    println!("cargo:rustc-link-lib=static=LLVMMC");
    println!("cargo:rustc-link-lib=static=LLVMCore");
    println!("cargo:rustc-link-lib=static=LLVMRemarks");
    println!("cargo:rustc-link-lib=static=LLVMBitstreamReader");
    println!("cargo:rustc-link-lib=static=LLVMBinaryFormat");
    println!("cargo:rustc-link-lib=static=LLVMFrontendOpenMP");
    println!("cargo:rustc-link-lib=static=LLVMCAS");
    println!("cargo:rustc-link-lib=static=LLVMSupport");
    println!("cargo:rustc-link-lib=static=LLVMDemangle");
}
