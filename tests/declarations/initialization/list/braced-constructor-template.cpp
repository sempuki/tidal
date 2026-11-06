// tidal-rules: ILC-1 DED-1
// tidal-expect: run
// tidal-note: Roadmap round 8, programmer reduction braced_init_constructor_template.cpp. A braced list with one element initializes a class through a constructor template, as parentheses would.
struct M {
  template <class V> M(V v) : value(v) {}
  int value;
};
int main() { M y{2}; return y.value == 2 ? 0 : 1; }
