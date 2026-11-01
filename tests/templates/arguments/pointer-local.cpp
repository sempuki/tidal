// tidal-rules: TARG-15
// tidal-expect: reject
// tidal-error-clang: non-type template argument is not a constant expression
// tidal-error-gcc: is not a valid template argument because it does not have
// tidal-error-edg: expression must have a constant value
template <int* P> struct Ptr {};
void use() {
  int local = 0;
  Ptr<&local> p;
}
