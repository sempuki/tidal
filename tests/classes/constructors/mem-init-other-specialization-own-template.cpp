// tidal-rules: CTOR-8 DEP-15
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer reduction specialization_base_of_same_template_mem_init.cpp. The partial specialization Cell<T*> has the base Cell<long>. Inside Cell<T*>, the template-id Cell<long> names another specialization (DEP-15), so the mem_init initializes the base and does not delegate (CTOR-8).
template <class T> struct Cell {
  int mark = 0;
  Cell() = default;
  Cell(int m) : mark(m) {}
};
template <class T> struct Cell<T*> : Cell<long> {
  Cell(int m) : Cell<long>(m + 1) {}
};
int main() {
  Cell<short*> c(6);
  return c.mark == 7 ? 0 : 1;
}
