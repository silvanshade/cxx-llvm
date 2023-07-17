#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "llvm/ADT/Triple.h"

// NOTE: these are global since cxx emits enum asserts without qualified names
using ArchType = ::llvm::Triple::ArchType;
using SubArchType = ::llvm::Triple::SubArchType;
using VendorType = ::llvm::Triple::VendorType;
using OSType = ::llvm::Triple::OSType;
using EnvironmentType = ::llvm::Triple::EnvironmentType;
using ObjectFormatType = ::llvm::Triple::ObjectFormatType;

namespace cxx_llvm::llvm::adt::triple {
using Triple = ::llvm::Triple;
using F = Triple;

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_align() noexcept -> size_t
{
  return cxx_memory::abi::cxx_abi_align<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_size() noexcept -> size_t
{
  return cxx_memory::abi::cxx_abi_size<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_default_constructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_default_constructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_copy_constructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_copy_constructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_move_constructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_move_constructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_destructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_destructible<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_copyable() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_trivially_copyable<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_movable() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_trivially_movable<F>();
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_destructible() noexcept -> bool
{
  return cxx_memory::abi::cxx_is_trivially_destructible<F>();
}

} // namespace cxx_llvm::llvm::adt::triple

namespace cxx_llvm::llvm::adt::triple {
[[gnu::always_inline]]
static inline auto
cxx_default_new(F* This [[clang::lifetimebound]]) noexcept -> void
{
  return cxx_memory::abi::cxx_default_new(This);
}

[[gnu::always_inline]]
static inline auto
cxx_copy_new(F* This [[clang::lifetimebound]], F const& that [[clang::lifetimebound]]) noexcept -> void
{
  return cxx_memory::abi::cxx_copy_new(This, that);
}

[[gnu::always_inline]]
static inline auto
cxx_move_new(F* This [[clang::lifetimebound]], F* that [[clang::lifetimebound]]) noexcept -> void
{
  // NOLINTNEXTLINE(hicpp-move-const-arg, performance-move-const-arg)
  F&& that_rvalue = ::std::move(*that);
  return cxx_memory::abi::cxx_move_new(This, ::std::forward<F&&>(that_rvalue));
}

[[gnu::always_inline]]
static inline auto
cxx_destruct(F* This [[clang::lifetimebound]]) -> void
{
  return cxx_memory::abi::cxx_destruct(This);
}

} // namespace cxx_llvm::llvm::adt::triple

namespace cxx_llvm::llvm::adt::triple {
[[gnu::always_inline]] [[gnu::const]]
static inline auto
placement_new_from_arch_vendor_os(
  F* This,
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
  F* This,
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
