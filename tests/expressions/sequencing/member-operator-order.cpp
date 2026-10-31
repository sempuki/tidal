// tidal-rules: SEQ-8
// tidal-expect: run
int order[4];
int n = 0;
int mark(int v) { order[n++] = v; return v; }
struct Num {
  int v;
  Num operator-(const Num& o) const { return {v - o.v}; }
};
Num left() { mark(1); return {5}; }
Num right() { mark(2); return {3}; }
int main() {
  Num r = left() - right();
  bool either = (order[0] == 1 && order[1] == 2) || (order[0] == 2 && order[1] == 1);
  return r.v == 2 && either ? 0 : 1;
}
