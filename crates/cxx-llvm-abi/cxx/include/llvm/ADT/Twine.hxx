#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "rust/cxx.h"

#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/Twine.h"

namespace cxx_llvm::llvm::adt::twine {
using Twine = ::llvm::Twine;
using F = Twine;

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

} // namespace cxx_llvm::llvm::adt::twine

namespace cxx_llvm::llvm::adt::twine {
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

} // namespace cxx_llvm::llvm::adt::twine

namespace cxx_llvm::llvm::adt::twine {
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
static inline auto
new_from_rust_str(rust::Str str [[clang::lifetimebound]]) noexcept -> F
{
  std::string_view&& view = { str.data(), str.size() };
  return { view };
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
static inline auto
new_from_rust_slice(rust::Slice<char const> const slice [[clang::lifetimebound]]) noexcept -> F
{
  ::llvm::StringRef&& string_ref = { slice.data(), slice.size() };
  return { string_ref };
}

} // namespace cxx_llvm::llvm::adt::twine
