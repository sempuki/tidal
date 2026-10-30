// tidal-rules: ACC-3
// tidal-expect: reject
class A {
  void f(int);
public:
  void f(double);
};
void g(A& a) { a.f(1); }
