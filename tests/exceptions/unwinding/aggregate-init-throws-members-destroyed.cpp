// tidal-rules: UNW-2
// tidal-expect: run
// tidal-note: The third initializer of an aggregate throws. The two members built before it are destroyed in reverse order, and the fourth is never built.
int seen[8];
int n = 0;
struct X { int id; X(int i) : id(i) {} ~X() { seen[n++] = id; } };
int boom() { throw 0; }
struct A { X a; X b; int c; X d; };
int main() {
  try { A x{X(1), X(2), boom(), X(4)}; (void)x; } catch (int) {}
  return n == 2 && seen[0] == 2 && seen[1] == 1 ? 0 : 1;
}
