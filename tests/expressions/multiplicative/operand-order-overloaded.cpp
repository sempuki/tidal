// tidal-rules: MUL-13
// tidal-expect: run
// tidal-note: Q-0005. GCC evaluates the right operand of an overloaded * first. Clang evaluates the left operand first.
struct Num { int v; };
Num operator*(Num a, Num b) { return {a.v * b.v}; }
int order[2];
int n = 0;
Num left() { order[n++] = 1; return {6}; }
Num right() { order[n++] = 2; return {3}; }
int main() {
  Num p = left() * right();
  bool either = (order[0] == 1 && order[1] == 2) || (order[0] == 2 && order[1] == 1);
  return p.v == 18 && n == 2 && either ? 0 : 1;
}
