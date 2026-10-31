// tidal-rules: VIRT-9
// tidal-expect: reject
// tidal-error-clang: does not look like a pure-specifier
// tidal-error-gcc: invalid pure specifier
// tidal-error-edg: badly-formed pure specifier
struct S { virtual void f() = 1; };
