// tidal-rules: CPT-27
// tidal-expect: reject
// tidal-error-clang: member reference base type 'int' is not a structure or union
// tidal-error-gcc: request for member .size. in
// tidal-error-edg: expression must have class type
// tidal-note: The requirement uses no template parameter, and the template is never instantiated.
template <class T> void check() { constexpr bool found = requires { int().size(); }; (void)found; }
