// tidal-rules: VC-4
// tidal-expect: run
struct S { int v; };
S make() { return {3}; }
int main() {
  const S& r = make();
  int&& m = make().v;
  return r.v == 3 && m == 3 ? 0 : 1;
}
