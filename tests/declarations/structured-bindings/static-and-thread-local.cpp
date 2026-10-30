// tidal-rules: SB-7
// tidal-expect: accept
int a[2] = {1, 2};
static auto [p, q] = a;
void f() {
  static auto [r, s] = a;
  thread_local auto [t, u] = a;
}
