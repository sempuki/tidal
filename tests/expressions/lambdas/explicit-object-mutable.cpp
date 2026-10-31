// tidal-rules: LAM-17
// tidal-expect: reject
// tidal-error-clang: explicit object parameter cannot be mutable
// tidal-error-gcc: .mutable. lambda specifier with explicit object parameter
// tidal-error-edg: mutable qualifier is not allowed on a lambda with an explicit
// tidal-note: Gap review 2026-10-31.
auto l = [](this auto self) mutable { return 1; };
