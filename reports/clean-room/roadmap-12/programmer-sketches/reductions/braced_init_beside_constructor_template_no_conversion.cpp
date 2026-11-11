// The ring's form: a constructor template whose parameter holds no
// template parameter and has no conversion from the argument adds no
// candidate (DED-28, ILC-3). tidal-fe stops with "constructor call".
struct S {
  int v;
  S(const int* x) : v(*x) {}
  template <bool C = true> S(double) : v(1) {}
};
int main() { int a = 0; S s{&a}; return s.v; }
