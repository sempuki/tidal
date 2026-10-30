// tidal-rules: IF-6
// tidal-expect: reject
struct P { int a, b; explicit operator bool() const { return a; } };
void f() {
  if (auto [a, b] = P{1, 2}) {}
}
