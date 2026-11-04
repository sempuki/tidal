// tidal-rules: ODR-10
// tidal-expect: run
// tidal-sources: inline-callee-defaults-differ.tu.cpp
// tidal-note: Propose exercise X13. The inline function has the same tokens in both units, but its call uses a different default argument in each. No compiler or linker reports it. The test accepts either value.
int pick(int v = 1);
inline int use() { return pick(); }
int use_elsewhere();
int pick(int v) { return v; }
int main() {
  int a = use(), b = use_elsewhere();
  return (a == 1 || a == 2) && (b == 1 || b == 2) ? 0 : 1;
}
