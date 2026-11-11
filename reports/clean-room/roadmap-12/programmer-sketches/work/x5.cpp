struct S {
  int v;
  S(const int* x) : v(*x) {}
  template <class T> S(T, double) : v(1) {}
};
int main() { int a = 0; S s{&a}; return s.v; }
