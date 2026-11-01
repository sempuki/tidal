// tidal-rules: DEP-4 INST-6
// tidal-expect: reject
// tidal-error-clang: zero size arrays
// tidal-error-gcc: zero-size array
// tidal-error-edg: greater than zero
template <int N> int last() {
  int items[N - 1] = {};
  return sizeof items;
}
int x = last<1>();
