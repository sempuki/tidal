// tidal-rules: DEL-4
// tidal-expect: reject
// tidal-error-clang: only special member functions and comparison operators may be defaulted
// tidal-error-gcc: cannot be defaulted
// tidal-error-edg: = default. can only appear on
void f() = default;
