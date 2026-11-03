// tidal-rules: CE-16
// tidal-expect: reject
// tidal-error-clang: undefined function .* cannot be used in a constant expression
// tidal-error-gcc: used before its definition
// tidal-error-edg: constexpr function .S::f. .* is not defined
// tidal-note: The constexpr member function is declared in its class and defined after the call.
struct S { constexpr int f() const; };
constexpr int v = S{}.f();
constexpr int S::f() const { return 1; }
