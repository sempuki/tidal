// tidal-rules: OVL-7
// tidal-expect: run
// tidal-note: P3-10. A declares operator= with a parameter of another class B. In a = B{3}, the member candidates of = include that function (OVL-7), and it takes the argument by an exact match. GCC, Clang, and EDG accept.
struct B { int k; };
struct A {
  int v = 0;
  A& operator=(B b) { v = b.k; return *this; }
};
int main() { A a; a = B{3}; return a.v - 3; }
