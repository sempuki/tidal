// tidal-rules: ILC-1
// tidal-expect: run
// tidal-note: Copy list initialization of a return value and of a function parameter, and direct list initialization in a new expression. Each list selects the constructor.
struct P {
  int x, y;
  P(int a, int b) : x(a), y(b * 10) {}
};
P make() { return {5, 6}; }
int sum(P p) { return p.x + p.y; }
int main() {
  P* q = new P{1, 1};
  bool ok = make().y == 60 && sum({2, 3}) == 32 && q->y == 10;
  delete q;
  return ok ? 0 : 1;
}
