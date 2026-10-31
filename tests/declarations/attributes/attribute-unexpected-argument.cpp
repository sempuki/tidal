// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-error-clang: noreturn. cannot have an argument list
// tidal-error-gcc: noreturn. attribute does not take any arguments
// tidal-error-edg: noreturn. does not take arguments
[[noreturn(1)]] void f();
