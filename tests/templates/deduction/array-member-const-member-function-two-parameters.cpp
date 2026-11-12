// tidal-rules: PRIM-12 PRIM-11 DED-6
// tidal-expect: run
// tidal-note: Roadmap round 15, programmer reduction const_member_function_array_member_deduction.cpp and probes member_function_array_member_deduction.cpp and const_local_array_deduction.cpp (from the tries of intervals.cpp). In the const member function head, cells means this->cells, and this points to a const Row (PRIM-11), so cells is an array of const int (PRIM-12). Both parameters of pick_first are not references, so the array becomes const int* (DED-6), and cells + 3 has the same type. P deduces const int* from both arguments. The member function that is not const deduces int*.
template <class P> constexpr P pick_first(P from, P) { return from; }
struct Row {
  int cells[3]{7, 8, 9};
  constexpr int head() const { return *pick_first(cells, cells + 3); }
  constexpr int head_mutable() { return *pick_first(cells, cells + 3); }
};
static_assert(Row{}.head() == 7 && Row{}.head_mutable() == 7);
int main() {
  const Row fixed;
  Row open;
  return fixed.head() + open.head_mutable() == 14 ? 0 : 1;
}
