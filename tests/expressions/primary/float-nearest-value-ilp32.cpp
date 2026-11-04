// tidal-rules: PRIM-21
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32. The same choice as on x86-64 (float-nearest-value.cpp, float-nearest-value-float.cpp). GCC keeps extra precision in constant evaluation on this target (MUL-17), so a cast rounds each value to its type first.
static_assert(static_cast<float>(0.1f) == 13421773.0f / 134217728.0f);
static_assert(static_cast<double>(0.1) == 3602879701896397.0 / 36028797018963968.0);
