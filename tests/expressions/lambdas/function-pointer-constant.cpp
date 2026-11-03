// tidal-rules: LAM-10 LAM-12
// tidal-expect: accept
// tidal-note: Write report W12. The conversion to a function pointer is usable in constant evaluation, and a call through the pointer is too when the call operator is constexpr.
constexpr int (*next)(int) = [](int x) { return x + 1; };
static_assert(next(4) == 5);
constexpr bool sorted(const int* v, int n, bool (*less)(int, int)) {
  for (int i = 1; i < n; ++i)
    if (less(v[i], v[i - 1])) return false;
  return true;
}
constexpr int values[3] = {1, 2, 3};
static_assert(sorted(values, 3, [](int a, int b) { return a < b; }));
