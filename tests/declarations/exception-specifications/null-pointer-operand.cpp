// tidal-rules: EXS-7
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: conversion from .std::nullptr_t. to .bool. is not allowed in a converted constant
// tidal-note: Q-0393. The operand is the null pointer literal. Its conversion to bool gives false and is not a narrowing conversion. GCC and EDG accept. Clang rejects, as the gap review of the text phase 8 wrote reads the C++23 text.
void f() noexcept(nullptr);
static_assert(!noexcept(f()));
