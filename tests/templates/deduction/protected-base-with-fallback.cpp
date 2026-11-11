// tidal-rules: DED-15 OVL-37 ACC-12
// tidal-expect: reject
// tidal-error-clang: cannot cast .* to its protected base class
// tidal-error-gcc: inaccessible base of
// tidal-error-edg: inaccessible
// tidal-note: I5-4. Guarded derives from Tag<int> through a protected base, and main is not a member or friend of Guarded. Deduction gives T = int, and the template wins over the fallback. The access to the base is checked after the choice (OVL-37), and the conversion fails (ACC-12).
template <class T> struct Tag {};
struct Guarded : protected Tag<int> {};
template <class T> int lift(Tag<T>*) { return 1; }
int lift(...) { return 2; }
int main() {
  Guarded g;
  return lift(&g) == 2 ? 0 : 1;
}
