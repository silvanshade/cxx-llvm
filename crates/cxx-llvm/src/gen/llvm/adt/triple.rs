#[cxx::bridge]
mod ffi {
    // NOTE: needed for the following enums (due to "last" cases)
    #![allow(unreachable_patterns)]

    #[namespace = "cxx_llvm::llvm::adt::triple"]
    #[derive(Debug)]
    #[repr(u32)]
    enum ArchType {
        UnknownArch,
        arm,
        armeb,
        aarch64,
        aarch64_be,
        aarch64_32,
        arc,
        avr,
        bpfel,
        bpfeb,
        csky,
        dxil,
        hexagon,
        loongarch32,
        loongarch64,
        m68k,
        mips,
        mipsel,
        mips64,
        mips64el,
        msp430,
        ppc,
        ppcle,
        ppc64,
        ppc64le,
        r600,
        amdgcn,
        riscv32,
        riscv64,
        sparc,
        sparcv9,
        sparcel,
        systemz,
        tce,
        tcele,
        thumb,
        thumbeb,
        x86,
        x86_64,
        xcore,
        nvptx,
        nvptx64,
        le32,
        le64,
        amdil,
        amdil64,
        hsail,
        hsail64,
        spir,
        spir64,
        spirv32,
        spirv64,
        kalimba,
        shave,
        lanai,
        wasm32,
        wasm64,
        renderscript32,
        renderscript64,
        ve,
        LastArchType = 59,
    }

    #[namespace = "cxx_llvm::llvm::adt::triple"]
    #[derive(Debug)]
    #[repr(u32)]
    enum SubArchType {
        NoSubArch,
        ARMSubArch_v9_3a,
        ARMSubArch_v9_2a,
        ARMSubArch_v9_1a,
        ARMSubArch_v9,
        ARMSubArch_v8_8a,
        ARMSubArch_v8_7a,
        ARMSubArch_v8_6a,
        ARMSubArch_v8_5a,
        ARMSubArch_v8_4a,
        ARMSubArch_v8_3a,
        ARMSubArch_v8_2a,
        ARMSubArch_v8_1a,
        ARMSubArch_v8,
        ARMSubArch_v8r,
        ARMSubArch_v8m_baseline,
        ARMSubArch_v8m_mainline,
        ARMSubArch_v8_1m_mainline,
        ARMSubArch_v7,
        ARMSubArch_v7em,
        ARMSubArch_v7m,
        ARMSubArch_v7s,
        ARMSubArch_v7k,
        ARMSubArch_v7ve,
        ARMSubArch_v6,
        ARMSubArch_v6m,
        ARMSubArch_v6k,
        ARMSubArch_v6t2,
        ARMSubArch_v5,
        ARMSubArch_v5te,
        ARMSubArch_v4t,
        AArch64SubArch_arm64e,
        KalimbaSubArch_v3,
        KalimbaSubArch_v4,
        KalimbaSubArch_v5,
        MipsSubArch_r6,
        PPCSubArch_spe,
    }

    #[namespace = "cxx_llvm::llvm::adt::triple"]
    #[derive(Debug)]
    #[repr(u32)]
    enum VendorType {
        UnknownVendor,
        Apple,
        PC,
        SCEI,
        Freescale,
        IBM,
        ImaginationTechnologies,
        MipsTechnologies,
        NVIDIA,
        CSR,
        Myriad,
        AMD,
        Mesa,
        SUSE,
        OpenEmbedded,
        LastVendorType = 14,
    }

    #[namespace = "cxx_llvm::llvm::adt::triple"]
    #[derive(Debug)]
    #[repr(u32)]
    enum OSType {
        UnknownOS,
        Ananas,
        CloudABI,
        Darwin,
        DragonFly,
        FreeBSD,
        Fuchsia,
        IOS,
        KFreeBSD,
        Linux,
        Lv2,
        MacOSX,
        NetBSD,
        OpenBSD,
        Solaris,
        Win32,
        ZOS,
        Haiku,
        Minix,
        RTEMS,
        NaCl,
        AIX,
        CUDA,
        NVCL,
        AMDHSA,
        PS4,
        PS5,
        ELFIAMCU,
        TvOS,
        WatchOS,
        DriverKit,
        Mesa3D,
        Contiki,
        AMDPAL,
        HermitCore,
        Hurd,
        WASI,
        Emscripten,
        ShaderModel,
        LastOSType = 38,
    }

    #[namespace = "cxx_llvm::llvm::adt::triple"]
    #[derive(Debug)]
    #[repr(u32)]
    enum EnvironmentType {
        UnknownEnvironment,
        GNU,
        GNUABIN32,
        GNUABI64,
        GNUEABI,
        GNUEABIHF,
        GNUX32,
        GNUILP32,
        CODE16,
        EABI,
        EABIHF,
        Android,
        Musl,
        MuslEABI,
        MuslEABIHF,
        MuslX32,
        MSVC,
        Itanium,
        Cygnus,
        CoreCLR,
        Simulator,
        MacABI,
        Pixel,
        Vertex,
        Geometry,
        Hull,
        Domain,
        Compute,
        Library,
        RayGeneration,
        Intersection,
        AnyHit,
        ClosestHit,
        Miss,
        Callable,
        Mesh,
        Amplification,
        LastEnvironmentType = 36,
    }

    #[namespace = "rust::llvm"]
    #[derive(Debug)]
    #[repr(u32)]
    enum ObjectFormatType {
        UnknownObjectFormat,
        COFF,
        DXContainer,
        ELF,
        GOFF,
        MachO,
        SPIRV,
        Wasm,
        XCOFF,
    }

    extern "C++" {
        include!("cxx-llvm-abi/cxx/include/llvm/ADT/Triple.hxx");
        include!("cxx-llvm-abi/cxx/include/llvm/ADT/Twine.hxx");

        #[namespace = "cxx_llvm::llvm::adt::triple"]
        type Triple = crate::ffi::llvm::adt::triple::Triple;

        type ArchType;

        type SubArchType;

        type VendorType;

        type OSType;

        type EnvironmentType;

        type ObjectFormatType;

        #[namespace = "cxx_llvm::llvm::adt::twine"]
        type Twine<'a> = crate::ffi::llvm::adt::twine::Twine<'a>;
    }

    #[namespace = "cxx_llvm::llvm::adt::triple"]
    unsafe extern "C++" {
        unsafe fn placement_new_from_arch_vendor_os(This: *mut Triple, arch: &Twine, vendor: &Twine, os: &Twine);

        unsafe fn placement_new_from_arch_vendor_os_environment(
            This: *mut Triple,
            arch: &Twine,
            vendor: &Twine,
            os: &Twine,
            environment: &Twine,
        );
    }
}
pub(crate) use self::ffi::*;
pub use self::ffi::{ArchType, EnvironmentType, OSType, ObjectFormatType, SubArchType, VendorType};
