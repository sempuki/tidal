// tidal-rules: NEW-39
// tidal-expect: reject
// tidal-error-clang: evaluated array bound -1 is negative
// tidal-error-gcc: call to non-.constexpr. function .*bad_array_new_length
// tidal-error-edg: too large for dynamic allocation
constexpr int make(int n) {
  int* p = new int[n];
  delete[] p;
  return 1;
}
static_assert(make(2) == 1);
constexpr int bad = make(-1);
