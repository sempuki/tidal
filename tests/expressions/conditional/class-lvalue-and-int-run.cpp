// tidal-rules: COND-16 COND-3
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction cond_class_lvalue_vs_int.cpp. A class lvalue meets an int that converts to the class through a converting constructor. The result is a prvalue of the class, made from the selected operand.
struct Depth {
  int m;
  Depth(int x) : m(x) {}
};
int pick(bool deep) {
  Depth shelf = 20;
  Depth d = deep ? 300 : shelf;
  return d.m;
}
int main() { return pick(true) == 300 && pick(false) == 20 ? 0 : 1; }
