// tidal-rules: PACK-31
// tidal-expect: reject
// tidal-error-clang: non-const reference cannot bind to bit-field
// tidal-error-gcc: cannot bind bit-field
// tidal-error-edg: taking the address of a bit field is not allowed
// tidal-note: The lvalue s.bits makes the second element an lvalue reference to int, which cannot bind to a bit-field.
struct S { int bits : 3; };
template <class... T> void relay(T&&...) {}
void use() { S s{}; relay(1, s.bits); }
