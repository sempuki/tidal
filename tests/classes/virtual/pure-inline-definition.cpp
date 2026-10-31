// tidal-rules: VIRT-9
// tidal-expect: reject
// tidal-error-clang: does not look like a pure-specifier
// tidal-error-gcc: pure-specifier on function-definition
// tidal-error-edg: pure specifier .* followed by a definition
struct S { virtual void f() = 0 {} };
