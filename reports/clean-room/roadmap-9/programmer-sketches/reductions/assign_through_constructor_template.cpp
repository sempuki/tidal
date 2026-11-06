// Assignment from a lambda to a class whose by-value operator= takes the
// class, made through a constructor template (a type-erased callable).
struct S {
  int n = 0;
  template <class F> S(F f) : n(f()) {}
  S& operator=(S o) { n = o.n; return *this; }
};
int main() { S s([] { return 1; }); s = [] { return 5; }; return s.n == 5 ? 0 : 1; }
