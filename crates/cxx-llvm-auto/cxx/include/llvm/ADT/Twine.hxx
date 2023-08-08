#pragma once

#include "cxx-llvm-auto/cxx/include/cxx-llvm-auto.hxx"
#include "rust/cxx.h"

#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/Twine.h"

namespace cxx_llvm::llvm::adt::twine {
CXX_AUTO_PRELUDE(Twine, ::llvm::Twine)
} // namespace cxx_llvm::llvm::adt::twine

namespace cxx_llvm::llvm::adt::twine {
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
static inline auto
new_from_rust_str(rust::Str str [[clang::lifetimebound]]) noexcept -> Self
{
  std::string_view&& view = { str.data(), str.size() };
  return { view };
}

[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
static inline auto
new_from_rust_slice(rust::Slice<char const> const slice [[clang::lifetimebound]]) noexcept -> Self
{
  ::llvm::StringRef&& string_ref = { slice.data(), slice.size() };
  return { string_ref };
}

} // namespace cxx_llvm::llvm::adt::twine
