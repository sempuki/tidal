// tidal-rules: MPTR-7
// tidal-expect: run
// tidal-note: One pointer names a virtual function and the other a function that is not virtual. The operator is !=, and either result is allowed.
struct V { virtual int f() { return 1; } int g() { return 2; } };
int main() {
  int (V::* p)() = &V::f;
  int (V::* q)() = &V::g;
  bool r = (p != q);
  return r || !r ? 0 : 1;
}
