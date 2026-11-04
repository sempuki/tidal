// tidal-rules: LIST-7
// tidal-expect: run
int order[4];
int n = 0;
int mark(int v) { order[n++] = v; return v; }
struct Pair { int a; int b; };
int main() {
  Pair p(mark(1), mark(2));
  return p.a == 1 && order[0] == 1 && order[1] == 2 ? 0 : 1;
}
