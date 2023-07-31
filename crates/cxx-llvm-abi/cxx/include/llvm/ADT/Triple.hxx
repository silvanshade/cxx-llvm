#pragma once

#include "cxx-llvm-abi/cxx/include/cxx-llvm-abi.hxx"

#include "llvm/ADT/Triple.h"

// NOTE: these are global since cxx emits enum asserts without qualified names
using TripleArchType = ::llvm::Triple::ArchType;
using TripleSubArchType = ::llvm::Triple::SubArchType;
using TripleVendorType = ::llvm::Triple::VendorType;
using TripleOSType = ::llvm::Triple::OSType;
using TripleEnvironmentType = ::llvm::Triple::EnvironmentType;
using TripleObjectFormatType = ::llvm::Triple::ObjectFormatType;

namespace cxx_llvm::llvm::adt::triple {
CXX_MEMORY_ABI_PRELUDE(Triple, ::llvm::Triple)
} // namespace cxx_llvm::llvm::adt::triple

namespace cxx_llvm::llvm::adt::triple {
[[gnu::always_inline]] [[gnu::const]]
static inline auto
placement_new_from_arch_vendor_os(
  Self* This,
  ::llvm::Twine const& arch,
  ::llvm::Twine const& vendor,
  ::llvm::Twine const& os
) noexcept -> void
{
  return cxx_memory::abi::cxx_placement_new(
    This,
    std::forward<::llvm::Twine const>(arch),
    std::forward<::llvm::Twine const>(vendor),
    std::forward<::llvm::Twine const>(os)
  );
}

[[gnu::always_inline]] [[gnu::const]]
static inline auto
placement_new_from_arch_vendor_os_environment(
  Self* This,
  ::llvm::Twine const& arch,
  ::llvm::Twine const& vendor,
  ::llvm::Twine const& os,
  ::llvm::Twine const& environment
) noexcept -> void
{
  return cxx_memory::abi::cxx_placement_new(
    This,
    std::forward<::llvm::Twine const>(arch),
    std::forward<::llvm::Twine const>(vendor),
    std::forward<::llvm::Twine const>(os),
    std::forward<::llvm::Twine const>(environment)
  );
}

} // namespace cxx_llvm::llvm::adt::triple
