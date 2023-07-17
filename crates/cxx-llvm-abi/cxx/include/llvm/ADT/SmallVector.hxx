#pragma once

#include "cxx-memory-abi/cxx/include/cxx-memory-abi.hxx"

#include "llvm/ADT/SmallVector.h"

namespace cxx_llvm::llvm::adt::small_vector {
template<typename T>
using F = ::llvm::SmallVector<T>;

} // namespace cxx_llvm::llvm::adt::small_vector
