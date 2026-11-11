// tidal-rules: CPY-1 CPY-4 OVL-24 CTOR-25
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization
// tidal-error-gcc: no matching function for call to .Cursor<false>::Cursor
// tidal-error-edg: has no suitable copy
// tidal-note: Roadmap round 13, programmer reduction copy_constructor_unsatisfied_constraint.cpp, the cause of the new verdict on ring_first_try.cpp. In Cursor<false>, the constructor from const Cursor<false>& is a copy constructor (CPY-1), so the class has no implicit copy constructor (CPY-4) and no implicit move constructor. The constraint of that copy constructor fails, so it is not viable (OVL-24), and the copy finds no constructor (CTOR-25).
template <bool Mutable> struct Cursor {
  int pos = 0;
  Cursor() = default;
  Cursor(const Cursor<false>& other) requires Mutable : pos(other.pos) {}
};
int main() {
  Cursor<false> a;
  Cursor<false> b = a;
  return b.pos;
}
