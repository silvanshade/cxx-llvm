#pragma once

#include "cxx-auto/cxx/include/cxx-auto.hxx"

#include "llvm/ADT/Hashing.h"
#include "llvm/Support/raw_ostream.h"

namespace cxx_auto::detection {
template<typename T>
concept is_llvm_hashable = requires(T const& val) { ::std::same_as<decltype(hash_value(val)), ::llvm::hash_code>; };

template<typename T>
concept has_method_llvm_dump = requires(T const& arg, llvm::raw_ostream& os) { //
  {
    arg.print(os)
  } -> ::std::same_as<void>;
};

template<typename T>
concept has_method_llvm_print = requires(T const& arg, llvm::raw_ostream& os) { //
  {
    arg.dump(os)
  } -> ::std::same_as<void>;
};
} // namespace cxx_auto::detection

namespace cxx_auto {
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

template<typename T>
requires(detection::has_method_llvm_dump<T>)
[[gnu::always_inline]]
static inline auto
cxx_debug(T const& This [[clang::lifetimebound]]) noexcept -> rust::String
{
  std::string str;
  llvm::raw_string_ostream os(str);
  This.dump(os);
  return rust::String::lossy(os.str());
}

template<typename T>
requires(detection::has_method_llvm_print<T>)
[[gnu::always_inline]]
static inline auto
cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> rust::String
{
  std::string str;
  llvm::raw_string_ostream os(str);
  This.print(os);
  return rust::String::lossy(std::to_string(This));
}

} // namespace cxx_auto
