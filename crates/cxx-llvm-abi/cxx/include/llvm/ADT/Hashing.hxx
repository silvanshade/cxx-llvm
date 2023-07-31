#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "llvm/ADT/Hashing.h"

namespace cxx_memory::abi::detection {
template<typename T>
concept is_llvm_hashable = requires(T const& val) { ::std::same_as<decltype(hash_value(val)), ::llvm::hash_code>; };
} // namespace cxx_memory::abi::detection

namespace cxx_memory::abi {
template<typename T>
requires(not detection::is_std_hashable<T>)
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]] [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_hashable() noexcept -> bool
{
  return detection::is_llvm_hashable<T>;
}

template<typename T>
requires(not detection::is_std_hashable<T>)
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]] [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_hash() noexcept -> bool
{
  return cxx_is_hashable<T>();
}

template<typename T>
requires(not detection::is_std_hashable<T> && detection::is_llvm_hashable<T>)
[[gnu::always_inline]] [[gnu::always_inline]]
static inline auto
cxx_hash(T const& This [[clang::lifetimebound]]) noexcept -> size_t
{
  return hash_value(This);
}

} // namespace cxx_memory::abi
