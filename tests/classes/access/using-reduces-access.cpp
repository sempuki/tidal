// tidal-rules: ACC-2
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: is a private member
// tidal-error-gcc: inaccessible within this context
// tidal-note: Q-0052. EDG 7.0 keeps the public access of the base member when a using-declaration redeclares it as private.
struct B { int x = 1; };
class D : public B {
private:
  using B::x;
};
int f(D& d) { return d.x; }
