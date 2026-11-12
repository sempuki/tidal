// tidal-rules: AUTO-11
// tidal-expect: reject
// tidal-error-clang: decltype\(auto\). not allowed here
// tidal-error-gcc: invalid use of .decltype\(auto\)
// tidal-error-edg: type name is not allowed
// tidal-note: RV10-4. decltype(auto) is the type of a functional cast. Only decltype of an expression can be that type, and every compiler rejects the placeholder.
int value = decltype(auto)(1);
