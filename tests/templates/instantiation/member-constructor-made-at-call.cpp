// tidal-rules: INST-5 INST-7
// tidal-expect: run
// tidal-note: P3-8. Base<Derived> is made while Derived is incomplete. The body of begin constructs Cursor<Derived>, whose constructor calls a member of Derived. The definitions of begin and of the constructor are made at the call in main, where Derived is complete (INST-5, INST-7). GCC, Clang, and EDG accept.
template <class S> struct Cursor {
  int v;
  explicit Cursor(S* p) : v(p->value()) {}
};
template <class D> struct Base {
  Cursor<D> begin() { return Cursor<D>(static_cast<D*>(this)); }
};
struct Derived : Base<Derived> { int value() { return 7; } };
int main() { Derived d; return d.begin().v - 7; }
