// tidal-rules: CLS-29 CLS-30
// tidal-expect: run
// tidal-note: An array of a standard-layout class and a const object of one are standard-layout. The array, its first element, and the element's first member share one address.
template <class A, class B> bool same_address(const A& a, const B& b) { return static_cast<const void*>(&a) == static_cast<const void*>(&b); }
struct Cell { int value; int next; };
int main() {
  Cell cells[3] = {};
  const Cell fixed{1, 2};
  return same_address(cells, cells[0]) && same_address(cells[0], cells[0].value) && same_address(fixed, fixed.value) ? 0 : 1;
}
