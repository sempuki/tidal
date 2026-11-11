struct S {
  int v;
  S(int x) : v(x) {}
  template <class T = int> S(T*, T*) : v(1) {}
};
int main() { S s = {0}; return s.v; }
