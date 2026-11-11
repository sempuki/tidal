// tidal-rules: TPL-13 OVL-35 CONV-14
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer probe converting_constructor_template_unconstrained_after_derived_pointer.cpp. The same as default-argument-converting-constructor-after-derived-pointer.cpp with no constraint on the constructor template. Handle<false> is built from a Circle* through a derived-to-base conversion (CONV-14), and the constructor template, whose parameter takes its default (TPL-13), then converts it to the Handle<true> parameter of area (OVL-35).
struct Shape { int size; };
struct Circle : Shape { int radius; };
template <bool Shared> struct Handle {
  Shape* target;
  explicit Handle(Shape* s) : target(s) {}
  template <bool S = Shared>
  Handle(const Handle<false>& other) : target(other.target) {}
};
int area(Handle<true> h) { return h.target->size; }
int main() {
  Circle c{{9}, 3};
  Handle<false> h(&c);
  return area(h) == 9 ? 0 : 1;
}
