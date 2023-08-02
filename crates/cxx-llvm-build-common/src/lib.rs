pub mod constants {
    pub const CMAKE_BUILD_MODE: &str = "ReleaseAssert";

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    pub const CMAKE_BUILD_TARGET: &str = "macosx-x86_64";
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    pub const CMAKE_BUILD_TARGET: &str = "macosx-arm64";
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub const CMAKE_BUILD_TARGET: &str = "linux-x86_64";
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    pub const CMAKE_BUILD_TARGET: &str = "linux-arm64";

    #[allow(non_snake_case)]
    pub fn NINJA_BUILD_DIR() -> String {
        format!("Ninja-{CMAKE_BUILD_MODE}")
    }
}

pub mod errors {
    pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
    pub type BoxResult<T> = Result<T, BoxError>;
}

pub mod traits {
    pub trait BuildExt: crate::traits::sealed::BuildExt {
        fn llvm_common_compiler(&mut self) -> &mut cc::Build;
        fn llvm_common_defines(&mut self) -> &mut cc::Build;
        fn llvm_common_flags(&mut self) -> &mut cc::Build;
    }

    impl BuildExt for cc::Build {
        fn llvm_common_compiler(&mut self) -> &mut cc::Build {
            self.compiler("clang++")
        }

        fn llvm_common_defines(&mut self) -> &mut cc::Build {
            self.define("_LIBCPP_ENABLE_THREAD_SAFETY_ANNOTATIONS", None)
                .define("_LIBCPP_ENABLE_CXX20_REMOVED_TYPE_TRAITS", None)
                .define("RUST_CXX_NO_EXCEPTIONS", None)
        }

        fn llvm_common_flags(&mut self) -> &mut cc::Build {
            self.flag_if_supported("-std=gnu++20")
                .flag_if_supported("-Werror")
                .flag_if_supported("-Wall")
                .flag_if_supported("-Wextra")
                .flag_if_supported("-Wthread-safety")
                .flag_if_supported("-Wthread-safety-beta")
                .flag_if_supported("-pedantic")
                .flag_if_supported("-Wno-ambiguous-reversed-operator")
                .flag_if_supported("-Wno-deprecated-anon-enum-enum-conversion")
                .flag_if_supported("-Wno-deprecated-builtins")
                .flag_if_supported("-Wno-deprecated-declarations")
                .flag_if_supported("-Wno-dollar-in-identifier-extension")
                .flag_if_supported("-Wno-nested-anon-types")
                .flag_if_supported("-Wno-unused-parameter")
                .flag_if_supported("-fno-exceptions")
                .flag_if_supported("-fno-rtti")
        }
    }

    pub(crate) mod sealed {
        pub trait BuildExt {}
        impl BuildExt for cc::Build {
        }
    }
}

pub mod util {
    use crate::errors::*;
    use std::path::PathBuf;

    pub fn get_path_from_env(var: &str, should_ignore: &mut bool) -> BoxResult<Option<PathBuf>> {
        if let Ok(path) = std::env::var(var) {
            if path.is_empty() {
                *should_ignore = true;
                return Ok(None);
            }
            let path = PathBuf::from(path);
            if !path.is_absolute() {
                return Err(format!(
                    "`{var}` specified but given path:\n\t\"{}\"\nis not absolute",
                    path.display()
                )
                .into());
            }
            if !path.exists() {
                return Err(format!(
                    "`{var}` specified but given path:\n\t\"{}\"\ndoes not exist",
                    path.display()
                )
                .into());
            }
            return Ok(Some(path));
        } else {
            return Ok(None);
        }
    }
}

pub mod prelude {
    pub use crate::{constants::*, errors::*, traits::*, util::*};
}
