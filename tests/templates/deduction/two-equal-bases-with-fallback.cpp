// tidal-rules: DED-15 OVL-37 CONV-17
// tidal-expect: reject
// tidal-error-clang: ambiguous conversion from derived class
// tidal-error-gcc: ambiguous base of
// tidal-error-edg: ambiguous
// tidal-note: I5-10 and I5-4, I5 program p02-two-equal-bases.cpp. Joined has two Tag<int> subobjects. Both give T = int, so deduction succeeds (DED-15), and the template is a better match than the fallback with an ellipsis. The conversion to the ambiguous base is checked after the choice (OVL-37), and it fails (CONV-17). The fallback does not make the call valid.
template <class T> struct Tag {};
struct Left : Tag<int> {};
struct Right : Tag<int> {};
struct Joined : Left, Right {};
template <class T> int lift(Tag<T>&) { return 1; }
int lift(...) { return 2; }
int main() {
  Joined j;
  return lift(j) == 2 ? 0 : 1;
}
