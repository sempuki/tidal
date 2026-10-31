// tidal-rules: CLS-12
// tidal-expect: reject
// tidal-error-clang: templates cannot be declared inside of a local class
// tidal-error-gcc: member template in local class
// tidal-error-edg: template declaration is not allowed here
// tidal-note: Gap review 2026-10-30.
void f() {
  struct L { template <class T> void g(T) {} };
}
