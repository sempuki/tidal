// tidal-rules: AUTO-8
// tidal-expect: reject
// tidal-error-clang: incomplete type .void
// tidal-error-gcc: deduced type .void. for .* is incomplete
// tidal-error-edg: cannot deduce .auto. type
void f();
auto a = f();
