// tidal-rules: DEP-11
// tidal-expect: run
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-error-gcc: there are no arguments to .operator T. that depend on a template parameter
// tidal-error-edg: identifier "operator int" is undefined
// tidal-note: The conversion function operator T of a dependent base is named without this->. The name is dependent, so its lookup waits for the instantiation and finds the base's function, as Clang does. GCC and EDG do not find it (Q-0248).
template <class T> struct Base { operator T() const { return T(7); } };
template <class T> struct Derived : Base<T> {
  T get() const { return operator T(); }
};
int main() { return Derived<int>{}.get() == 7 ? 0 : 1; }
