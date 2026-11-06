// tidal-rules: COND-3 COND-8
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction cond_class_lvalue_vs_prvalue_constant.cpp. In constant evaluation, a class lvalue and a prvalue of the same class give a prvalue, which is copied from the lvalue when the lvalue is selected.
struct Pair { int a, b; };
constexpr int sum(bool keep) {
  Pair local{1, 2};
  Pair p = keep ? local : Pair{10, 20};
  return p.a + p.b;
}
static_assert(sum(true) == 3);
static_assert(sum(false) == 30);
int main() { return sum(false) == 30 ? 0 : 1; }
