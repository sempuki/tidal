// X24 probe k1: continue and break before an initialized declaration
// in the same body. The declaration is in scope at the end of the body.
struct Noisy { int v; Noisy(int x) : v(x) {} ~Noisy() {} };
constexpr int f() {
  int n = 0;
  for (int i = 0; i < 4; ++i) {
    if (i % 2) continue;      // leaves the body; skips y
    if (i == 3) break;
    int y = i;                // initialized declaration after the jump
    n += y;
  }
  return n;
}
static_assert(f() == 2);
int g(int k) {
  int n = 0;
  for (int i = 0; i < 4; ++i) {
    if (i == k) continue;
    Noisy z{i};               // class type with a destructor
    n += z.v;
  }
  return n;
}
int main() { return g(1) == 5 ? 0 : 1; }
