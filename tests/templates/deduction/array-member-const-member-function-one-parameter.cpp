// tidal-rules: PRIM-12 DED-6
// tidal-expect: run
// tidal-note: Roadmap round 15, programmer reduction const_member_function_array_member_decay.cpp (from the tries of intervals.cpp). In a const member function, the array member cells has const elements (PRIM-12). The parameter of keep is not a reference, so the array becomes a pointer to its element type and keeps the const (DED-6): P is const int*. In a member function that is not const, P is int*.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class P> constexpr P keep(P p) { return p; }
struct Row {
  int cells[3]{7, 8, 9};
  constexpr bool decays_to_const() const { return same<decltype(keep(cells)), const int*>; }
  constexpr bool decays_plain() { return same<decltype(keep(cells)), int*>; }
  constexpr int last() const { return keep(cells)[2]; }
};
static_assert(Row{}.decays_to_const() && Row{}.decays_plain());
static_assert(Row{}.last() == 9);
int main() {
  const Row fixed;
  return fixed.decays_to_const() && fixed.last() == 9 ? 0 : 1;
}
