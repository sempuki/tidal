// tidal-rules: ACC-3
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
class A {
  void f(int);
public:
  void f(double);
};
void g(A& a) { a.f(1); }
