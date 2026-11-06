// tidal-rules: INIT-9 CTOR-2
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction array_of_class_with_constructor_template.cpp. Default initialization of an array runs the defaulted default constructor for each element. The class also has a constructor template, which default initialization does not use.
struct Slot {
  int* target = nullptr;
  Slot() = default;
  template <class U> Slot(U) {}
};
int main() {
  Slot slots[3];
  return slots[0].target == nullptr && slots[2].target == nullptr ? 0 : 1;
}
