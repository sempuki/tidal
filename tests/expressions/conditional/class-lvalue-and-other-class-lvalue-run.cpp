// tidal-rules: COND-16 COND-3
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction cond_class_lvalue_vs_convertible_class.cpp. A const lvalue of one class meets an lvalue of a second class, which has a converting constructor from the first. Only the first operand converts, so the result is a prvalue of the second class.
struct Key {
  int id;
  constexpr explicit Key(int i) : id(i) {}
};
struct Entry {
  int id = -1;
  Entry(int i) : id(i) {}
  Entry(Key k) : id(k.id + 100) {}
};
int look(bool cached) {
  Entry e = 4;
  const Key k{5};
  Entry r = cached ? e : k;
  return r.id;
}
int main() { return look(true) == 4 && look(false) == 105 ? 0 : 1; }
