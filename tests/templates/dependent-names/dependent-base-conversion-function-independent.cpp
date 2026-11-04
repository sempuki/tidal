// tidal-rules: DEP-12
// tidal-expect: reject
// tidal-error-clang: use of undeclared .operator int.
// tidal-error-gcc: there are no arguments to .operator int. that depend on a template parameter
// tidal-error-edg: identifier "operator int" is undefined
// tidal-note: The conversion function name operator int does not depend on T, so lookup at the definition does not search the dependent base Base<T> (Q-0248).
template <class T> struct Base { operator int() const { return 7; } };
template <class T> struct Derived : Base<T> {
  int get() const { return operator int(); }
};
int main() { return Derived<int>{}.get() == 7 ? 0 : 1; }
