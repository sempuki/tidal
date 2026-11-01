// tidal-rules: DEP-12
// tidal-expect: reject
// tidal-expect-clang: run
// tidal-error-gcc: there are no arguments to .operator T. that depend on a template parameter
// tidal-error-edg: identifier "operator int" is undefined
// tidal-note: The conversion function operator T of a dependent base is named without this->. Clang finds the function at the instantiation, GCC and EDG do not find it (Q-0248).
template <class T> struct Base { operator T() const { return T(7); } };
template <class T> struct Derived : Base<T> {
  T get() const { return operator T(); }
};
int main() { return Derived<int>{}.get() == 7 ? 0 : 1; }
