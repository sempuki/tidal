struct S {
  int v;
  S(int x) : v(x) {}
  template <class T> S(T* p) : v(*p) {}
};
int main() { S s{0}; return s.v; }
