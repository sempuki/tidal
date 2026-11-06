// tidal-rules: REF-1 ICS-11
// tidal-expect: run
// tidal-note: Decision 0059, item F2 (I3-5), with the round 9 writer probe i3-5-qualification-direct-binding.cpp. An int* lvalue binds a const int* const& directly, so the reference and the pointer are one object. An int* prvalue binds a temporary, and the overload with an rvalue reference wins for it.
int x = 0;
int* p = &x;
int* make() { return &x; }
int f(const int* const&) { return 1; }
int f(const int*&&) { return 2; }
int main() {
  const int* const& r = p;
  bool same_object = &r == &p;
  return same_object && f(p) == 1 && f(make()) == 2 ? 0 : 1;
}
