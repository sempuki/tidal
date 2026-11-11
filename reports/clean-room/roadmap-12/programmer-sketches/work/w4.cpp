struct S {
  int v;
  S(int x) : v(x) {}
  template <class T> S(T, T) : v(0) {}
};
int main() { S s{1, 1}; return s.v; }
