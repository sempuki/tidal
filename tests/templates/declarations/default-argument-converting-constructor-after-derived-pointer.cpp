// tidal-rules: TPL-13 OVL-35 CONV-14
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer reduction converting_constructor_template_after_derived_pointer_construction.cpp (the cause of the TR-13 stop of r10-list-first-try). Handle<false> is built from a Circle* that converts to Shape* (CONV-14). The conversion in that construction changes nothing in the later call: the constructor template takes Shared from its default (TPL-13), its constraint holds for Handle<true>, and it converts the Handle<false> argument, so area has a viable function (OVL-35).
struct Shape { int size; };
struct Circle : Shape { int radius; };
template <bool Shared> struct Handle {
  Shape* target;
  explicit Handle(Shape* s) : target(s) {}
  template <bool S = Shared> requires S
  Handle(const Handle<false>& other) : target(other.target) {}
};
int area(Handle<true> h) { return h.target->size; }
int main() {
  Circle c{{7}, 2};
  Handle<false> h(&c);
  return area(h) == 7 ? 0 : 1;
}
