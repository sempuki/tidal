// tidal-rules: AUTO-11
// tidal-expect: reject
// tidal-error-clang: decltype\(auto\). not allowed here
// tidal-error-gcc: invalid use of .decltype\(auto\)
// tidal-error-edg: expected an identifier
// tidal-note: RV10-1. decltype(auto) names a base. Only decltype of an expression can name a base, and every compiler rejects the placeholder there.
struct Base {};
struct Derived : decltype(auto) {};
