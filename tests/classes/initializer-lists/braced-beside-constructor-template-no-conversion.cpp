// tidal-rules: ILC-3 DED-28
// tidal-expect: run
// tidal-note: Roadmap round 12, programmer reduction braced_init_beside_constructor_template_no_conversion.cpp, the form of an iterator with a converting constructor template. The template parameter Shared takes its default, and the function parameter of type double holds no template parameter. A pointer has no implicit conversion to double, so the template adds no candidate (DED-28), and the braced list calls Gauge(const int*) (ILC-3).
struct Gauge {
  int level;
  Gauge(const int* p) : level(*p) {}
  template <bool Shared = true> Gauge(double) : level(-1) {}
};
int main() {
  int reading = 12;
  Gauge g{&reading};
  return g.level == 12 ? 0 : 1;
}
