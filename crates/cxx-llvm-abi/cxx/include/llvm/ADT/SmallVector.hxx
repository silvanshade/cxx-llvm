#pragma once

#include "cxx-llvm-abi/cxx/include/cxx-llvm-abi.hxx"

#include "llvm/ADT/SmallVector.h"

namespace cxx_llvm::llvm::adt::small_vector {
template<typename T>
using F = ::llvm::SmallVector<T>;
} // namespace cxx_llvm::llvm::adt::small_vector
