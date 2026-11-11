// tidal-rules: DEP-24 DEP-11
// tidal-expect: run
// tidal-note: I6-11. get is a member template of the dependent base T, so lookup at the definition finds nothing, and the template keyword after this-> makes the following < open an argument list (DEP-24). The name is looked up at the instantiation (DEP-11). GCC, Clang, and EDG accept, and the program returns 0.
struct Base {
  template <int N> int get() const { return N - 3; }
};
template <class T> struct Derived : T {
  int run() const { return this->template get<3>(); }
};
int main() { return Derived<Base>{}.run(); }
