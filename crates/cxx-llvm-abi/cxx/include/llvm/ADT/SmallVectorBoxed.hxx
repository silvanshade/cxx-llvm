#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "llvm/ADT/SmallVector.h"

namespace cxx_llvm::llvm::adt::small_vector_boxed {
template<typename T>
class SmallVectorBoxed
{
private:
  ::std::unique_ptr<void, void (*)(void*)> m_ptr;
  ::llvm::SmallVectorImpl<T>* m_impl;

  template<unsigned int N>
  static auto erase(::llvm::SmallVector<T, N>* ptr) -> ::std::unique_ptr<void, void (*)(void*)>
  {
    return ::std::unique_ptr<void, void (*)(void*)>(static_cast<void*>(ptr), [](void* ptr) {
      // NOLINTNEXTLINE(cppcoreguidelines-owning-memory)
      delete static_cast<::llvm::SmallVector<T, N>*>(ptr);
    });
  }

  template<unsigned int N>
  explicit SmallVectorBoxed(std::unique_ptr<::llvm::SmallVector<T, N>> ptr) noexcept
    : m_ptr{ erase(ptr.release()) }
    , m_impl{ static_cast<::llvm::SmallVectorImpl<T>*>(m_ptr.get()) }
  {
  }

public:
  template<unsigned int N>
  explicit SmallVectorBoxed(::llvm::SmallVector<T, N>&& vec) noexcept
    : SmallVectorBoxed{ ::std::make_unique<::llvm::SmallVector<T, N>>(::std::move(vec)) }
  {
  }

  SmallVectorBoxed(SmallVectorBoxed const&) = delete;
  SmallVectorBoxed(SmallVectorBoxed&&) noexcept = default;

  ~SmallVectorBoxed() noexcept = default;

  auto operator=(SmallVectorBoxed const&) -> SmallVectorBoxed& = delete;
  auto operator=(SmallVectorBoxed&&) noexcept -> SmallVectorBoxed& = default;

  auto as_ref_small_vector_impl() const& noexcept -> ::llvm::SmallVectorImpl<T> const& { return *m_impl; }
  auto as_pin_small_vector_impl() & noexcept -> ::llvm::SmallVectorImpl<T>& { return *m_impl; }
};

} // namespace cxx_llvm::llvm::adt::small_vector_boxed

namespace cxx_llvm::llvm::adt::small_vector_boxed {
template<typename T>
using F = SmallVectorBoxed<T>;

} // namespace cxx_llvm::llvm::adt::small_vector_boxed
