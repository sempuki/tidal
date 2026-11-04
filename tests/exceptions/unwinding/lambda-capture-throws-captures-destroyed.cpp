// tidal-rules: UNW-2
// tidal-expect: run
// tidal-note: The third capture of a lambda throws. The two captures of the closure object built before it are destroyed in reverse order, and then the local p.
int seen[8];
int n = 0;
struct X { int id; X(int i) : id(i) {} X(const X& o) : id(o.id + 10) {} ~X() { seen[n++] = id; } };
int boom() { throw 0; }
int main() {
  try { X p(5); auto l = [q = X(6), p, z = boom()] { }; (void)l; } catch (int) {}
  return n == 3 && seen[0] == 15 && seen[1] == 6 && seen[2] == 5 ? 0 : 1;
}
