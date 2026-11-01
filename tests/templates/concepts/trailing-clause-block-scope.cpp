// tidal-rules: CPT-14
// tidal-expect: reject
// tidal-error-clang: non-templated function cannot have a requires clause
// tidal-error-gcc: constraints on a non-templated function
// tidal-error-edg: requires-clause is not allowed here
template <class T> int outer() {
  int inner() requires (sizeof(T) > 1);
  return 0;
}
int value = outer<int>();
