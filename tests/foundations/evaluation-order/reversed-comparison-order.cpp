// tidal-rules: SEQ-8
// tidal-expect: run
int order[4];
int n = 0;
int mark(int v) { order[n++] = v; return v; }
struct A { int v; };
struct B { int v; };
bool operator==(B b, A a) { return a.v == b.v; }
A make_a() { mark(1); return {1}; }
B make_b() { mark(2); return {1}; }
int main() {
  bool same = make_a() == make_b();
  bool either = (order[0] == 1 && order[1] == 2) || (order[0] == 2 && order[1] == 1);
  return same && either ? 0 : 1;
}
