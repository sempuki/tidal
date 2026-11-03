// tidal-rules: IF-22
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .g
// tidal-error-gcc: too few arguments to function .void g\(int\)
// tidal-error-edg: too few arguments in function call
// tidal-note: The discarded call names a declared function with too few arguments. The call does not depend on T.
void g(int);
template <class T>
void f(T) {
  if constexpr (false) {
    g();
  }
}
