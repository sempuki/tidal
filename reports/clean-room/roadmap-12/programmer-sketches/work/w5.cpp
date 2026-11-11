struct S {
  int v;
  template <class T> S(T, T) : v(1) {}
};
int main() { S s{0}; return s.v; }
