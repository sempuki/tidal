// tidal-rules: CTOR-15
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer reduction two_specializations_of_one_base_template_mem_init.cpp. Slot<0, int> and Slot<1, int> are two bases that are specializations of one class template. Each mem_init names a different base by its template-id, so neither base is named two times.
template <int K, class T> struct Slot {
  T held;
  Slot(T v) : held(v) {}
};
struct Couple : Slot<0, int>, Slot<1, int> {
  Couple(int a, int b) : Slot<0, int>(a), Slot<1, int>(b) {}
};
int main() {
  Couple c(5, 9);
  const Slot<0, int>& first = c;
  const Slot<1, int>& second = c;
  return (first.held == 5 && second.held == 9) ? 0 : 1;
}
