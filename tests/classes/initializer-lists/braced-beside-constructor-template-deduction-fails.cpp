// tidal-rules: ILC-3 DED-1 DED-35
// tidal-expect: run
// tidal-note: Roadmap round 12, programmer reduction braced_init_beside_failed_constructor_template.cpp (from ring_fifth_try.cpp). The class has no initializer-list constructor, so every constructor is a candidate with the element 9 as the argument (ILC-3). Deduction of T* from an int fails, so the template adds no candidate, and Gauge(int) is chosen. The braced form gives the same result as the form with parentheses.
struct Gauge {
  int level;
  Gauge(int n) : level(n) {}
  template <class T> Gauge(T* source) : level(*source + 100) {}
};
int main() {
  Gauge braced{9};
  Gauge paren(9);
  return braced.level == 9 && paren.level == 9 ? 0 : 1;
}
