#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"
#include "rust/cxx.h"

#include "llvm/ADT/StringRef.h"

namespace cxx_llvm::llvm::adt::string_ref {
CXX_MEMORY_ABI_PRELUDE(StringRef, ::llvm::StringRef)
} // namespace cxx_llvm::llvm::adt::string_ref

namespace cxx_llvm::llvm::adt::string_ref {
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
static inline auto
new_from_rust_str(rust::Str str [[clang::lifetimebound]]) noexcept -> Type
{
  std::string_view&& view = { str.data(), str.size() };
  return { view };
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
static inline auto
new_from_rust_slice(rust::Slice<char const> const slice [[clang::lifetimebound]]) noexcept -> Type
{
  return { slice.data(), slice.size() };
}

[[nodiscard]] [[gnu::always_inline]]
static inline auto
as_slice(Type This [[clang::lifetimebound]]) noexcept -> rust::Slice<char const>
{
  return { This.data(), This.size() };
}

} // namespace cxx_llvm::llvm::adt::string_ref
