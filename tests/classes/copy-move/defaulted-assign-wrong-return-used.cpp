// tidal-rules: CPY-17
// tidal-expect: reject
// tidal-error-clang: must return .S &|must be an lvalue reference type
// tidal-error-gcc: does not match the expected signature
// tidal-error-edg: invalid type for defaulted assignment operator
struct S { void operator=(const S&) = default; };
void f(S& a, const S& b) { a = b; }
