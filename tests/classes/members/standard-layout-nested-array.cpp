// tidal-rules: CLS-29 CLS-30
// tidal-expect: run
// tidal-note: A standard-layout class whose first member is an array of a standard-layout class, and a volatile object of that class. The object, the array, its first element, and the element's first member share one address.
template <class A, class B> bool same_address(const volatile A& a, const volatile B& b) {
  return static_cast<const volatile void*>(&a) == static_cast<const volatile void*>(&b);
}
struct Pair { short first; short second; };
struct Row { Pair cells[2]; int* next; };
int main() {
  Row row{};
  volatile Row shared{};
  return same_address(row, row.cells) && same_address(row.cells, row.cells[0]) &&
                 same_address(row.cells[0], row.cells[0].first) && same_address(shared, shared.cells[0].first)
             ? 0
             : 1;
}
