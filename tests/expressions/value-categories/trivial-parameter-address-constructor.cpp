// tidal-rules: VC-7
// tidal-expect: run
// tidal-note: The parameter belongs to a constructor. Both answers are allowed.
struct P { int a; int b; };
struct Holder {
  bool same;
  Holder(P p, const P* caller) : same(&p == caller) {}
};
int main() {
  P p{1, 2};
  Holder h(p, &p);
  return (h.same || !h.same) ? 0 : 1;
}
