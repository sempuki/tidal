// tidal-rules: INST-5 INST-7
// tidal-expect: run
// tidal-note: P3-7. Base<Derived> is made while Derived is incomplete. Its member begin has a body whose return type Cursor<Derived> reads Derived::value_type. The instantiation of Base<Derived> makes the declaration of begin and not its definition (INST-5), and nothing calls begin, so the definition is never made (INST-7). GCC, Clang, and EDG accept.
template <class S> struct Cursor { typename S::value_type current; };
template <class D> struct Base {
  Cursor<D> begin() { return Cursor<D>{}; }
};
struct Derived : Base<Derived> { using value_type = int; };
int main() { Derived d; (void)d; return 0; }
