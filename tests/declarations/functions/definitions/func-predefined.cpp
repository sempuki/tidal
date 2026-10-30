// tidal-rules: FN-5
// tidal-expect: run
constexpr int length(const char* s) { int n = 0; while (s[n]) ++n; return n; }
int check() {
  static_assert(sizeof(__func__) == 6);
  return __func__[0] == 'c' && length(__func__) == 5 ? 0 : 1;
}
int main() { return check(); }
