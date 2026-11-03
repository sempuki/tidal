// tidal-rules: NEW-39 CE-8
// tidal-expect: reject
// tidal-error-clang: too small to hold 3 explicitly initialized elements
// tidal-error-gcc: call to non-.constexpr. function .*bad_array_new_length
// tidal-error-edg: too small for initializer
constexpr int make(int n) {
  int* p = new int[n]{1, 2, 3};
  int v = p[0];
  delete[] p;
  return v;
}
static_assert(make(3) == 1);
constexpr int bad = make(2);
