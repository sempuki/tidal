// tidal-rules: SEQ-2
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
struct P { int a; int b; };
int main() {
  int i = 0;
  P p{i++, i++};
  return p.a == 0 && p.b == 1 ? 0 : 1;
}
