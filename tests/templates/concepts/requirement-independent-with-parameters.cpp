// tidal-rules: CPT-27
// tidal-expect: reject
// tidal-error-clang: member reference base type 'int' is not a structure or union
// tidal-error-gcc: request for member .size. in
// tidal-error-edg: expression must have class type
// tidal-note: The requires expression has a parameter of type T, but the requirement does not use it.
template <class T> void check() { constexpr bool found = requires (T t) { int().size(); }; (void)found; }
