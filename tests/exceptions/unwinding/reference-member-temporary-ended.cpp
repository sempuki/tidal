// tidal-rules: UNW-2
// tidal-expect: run
// tidal-expect-gcc-14: run-fail
// tidal-note: The reference member r of the aggregate extends the life of a temporary. When a later initializer throws, the temporary ends after the member built after it, in reverse order. GCC 14 gives a different order or count, and the program returns 1.
int seen[8];
int n = 0;
struct X { int id; X(int i) : id(i) {} ~X() { seen[n++] = id; } };
int boom() { throw 0; }
struct L { X a; const X& r; int c; };
int main() {
  try { L y{X(7), X(8), boom()}; (void)y; } catch (int) {}
  return n == 2 && seen[0] == 8 && seen[1] == 7 ? 0 : 1;
}
