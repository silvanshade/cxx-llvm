#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "llvm/ADT/SmallVector.h"

namespace cxx_llvm::llvm::adt::small_vector_impl {
template<typename T>
using F = ::llvm::SmallVectorImpl<T>;

} // namespace cxx_llvm::llvm::adt::small_vector_impl
