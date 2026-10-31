// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: noreturn. attribute only applies to functions
// tidal-error-edg: attribute .noreturn. does not apply here
// tidal-note: Q-0021. GCC ignores [[noreturn]] on a variable with a warning; Clang and EDG reject it.
[[noreturn]] int x;
