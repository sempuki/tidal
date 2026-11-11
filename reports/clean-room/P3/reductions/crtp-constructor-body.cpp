// tidal-fe rejects with TYN-7 at line 7 (member access into an incomplete class).
// g++ and clang++ accept, exit 0. Derived is complete where main calls begin().
// A plain member function with the same body (not a constructor) is accepted.
template <class S> struct Cursor {
    S* s;
    int v;
    explicit Cursor(S* p) : s(p), v(0) { v = p->value(); }
};
template <class D> struct Base {
    Cursor<D> begin() { return Cursor<D>(static_cast<D*>(this)); }
};
struct Derived : Base<Derived> { int value() { return 0; } };
int main() { Derived d; return d.begin().v; }
