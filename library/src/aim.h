#pragma once

#include <iostream>
#include "nameof.hpp"

// https://github.com/dtolnay/cxx/issues/796
using c_void = void;

// helper for debugging on the c++ side
template <class T>
void dump(T const *t) {
    auto p = reinterpret_cast<unsigned long const *>(t);
    for (size_t n = 0 ; n < sizeof(T) ; ++n)
        printf("0x%012lx ", p[n]);
    printf("\n");
}

template<typename T>
struct is_pointer { static const bool value = false; };

template<typename T>
struct is_pointer<T*> { static const bool value = true; };

template<typename T>
void printTypeInfo(const char * desc, T obj) {
    if (is_pointer<T>::value) {
        std::cout << desc << ", typeof(" << obj << ") = " << NAMEOF_TYPE(T) << std::endl;
    } else {
        std::cout << desc << ", typeof(_) = " << NAMEOF_TYPE(T) << std::endl;
    }
}
