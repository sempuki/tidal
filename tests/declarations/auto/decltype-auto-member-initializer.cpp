// tidal-rules: AUTO-11
// tidal-expect: reject
// tidal-error-clang: decltype\(auto\). not allowed here
// tidal-error-gcc: decltype\(auto\). is not a direct base
// tidal-error-edg: expected an identifier
// tidal-note: RV10-1. decltype(auto) names the base in a member initializer list. Only decltype of an expression can stand there, and every compiler rejects the placeholder.
struct Base { Base(int) {} };
struct Derived : Base { Derived() : decltype(auto)(1) {} };
