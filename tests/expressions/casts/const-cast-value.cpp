// tidal-rules: CAST-4
// tidal-expect: reject
// tidal-error-clang: const_cast to .int.* which is not a reference
// tidal-error-gcc: invalid use of .const_cast. with type .int.
// tidal-error-edg: type in a const_cast must be a pointer
int f(const int v) { return const_cast<int>(v); }
