struct S {
  int v;
  S(int x) : v(x) {}
  template <class T> S(T* p) : v(*p) {}
};
int main() { int k = 0; S s{k}; return s.v; }
