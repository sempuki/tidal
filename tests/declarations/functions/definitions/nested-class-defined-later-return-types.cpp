// tidal-rules: FN-12
// tidal-expect: run
// tidal-note: RV10-R-2. Inner is defined after Outer with a qualified name. Its member and friend functions check their types where the definition of Inner ends, so Inner and Holder<Inner> are complete there.
template <class T> struct Holder { T value; };
struct Outer { struct Inner; };
struct Outer::Inner {
  int k;
  Inner next() const { return {k + 1}; }
  Holder<Inner> wrap() const { return {*this}; }
  friend Holder<Inner> pack(Inner i) { return {i}; }
};
int main() {
  Outer::Inner start{3};
  bool ok = start.next().k == 4 && start.wrap().value.k == 3 && pack(start).value.k == 3;
  return ok ? 0 : 1;
}
