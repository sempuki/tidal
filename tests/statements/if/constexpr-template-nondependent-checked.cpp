// tidal-rules: IF-13
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .never_declared
// tidal-error-gcc: declaration of .never_declared. must be available
// tidal-error-edg: identifier .never_declared. is undefined
// tidal-note: Gap review 2026-10-30.
template <class T>
void f(T) {
  if constexpr (false) {
    never_declared();
  }
}
