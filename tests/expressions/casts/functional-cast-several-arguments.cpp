// tidal-rules: CAST-10
// tidal-expect: run
struct P { int a; int b; P(int x, int y) : a(x), b(y) {} };
int main() {
  P p = P(1, 2);
  return p.a + p.b == 3 ? 0 : 1;
}
