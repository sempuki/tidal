// tidal-rules: CX-1
// tidal-expect: reject
// tidal-error-clang: cannot assign to variable .n. with const-qualified type
// tidal-error-gcc: assignment of read-only variable .n
// tidal-error-edg: must be a modifiable lvalue
constexpr int n = 1;
void f() { n = 2; }
