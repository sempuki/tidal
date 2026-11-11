// tidal-rules: DED-15 OVL-37 ACC-12
// tidal-expect: reject
// tidal-error-clang: cannot cast .* to its private base class
// tidal-error-gcc: inaccessible base of
// tidal-error-edg: inaccessible
// tidal-note: I5-4, I5 program p08-private-base.cpp. Hidden derives from Tag<int> through a private base. Deduction searches the bases without access, so T = int, and the template is a better match than the fallback with an ellipsis. The access to the base is checked after the choice (OVL-37), and the conversion fails (ACC-12).
template <class T> struct Tag {};
struct Hidden : private Tag<int> {};
template <class T> int lift(Tag<T>&) { return 1; }
int lift(...) { return 2; }
int main() {
  Hidden h;
  return lift(h) == 2 ? 0 : 1;
}
