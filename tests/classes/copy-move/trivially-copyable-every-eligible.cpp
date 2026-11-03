// tidal-rules: CPY-19 CPY-9
// tidal-expect: run
// tidal-note: Twin has two eligible copy constructors. The one for a const source is trivial, and the one for a non-const source is user-provided. CPY-19 needs every eligible one to be trivial, so Twin is not trivially copyable. No core-language check shows the whole property, so the test shows that both constructors are used.
struct Twin {
  int value = 0;
  bool by_user = false;
  Twin() = default;
  Twin(const Twin&) = default;
  Twin(Twin& other) : value(other.value), by_user(true) {}
};
int main() {
  Twin a;
  const Twin& c = a;
  Twin from_const(c);
  Twin from_plain(a);
  return !from_const.by_user && from_plain.by_user ? 0 : 1;
}
