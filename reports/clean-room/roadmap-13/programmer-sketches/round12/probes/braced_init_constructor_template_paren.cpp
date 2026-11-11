// Probe: the braced reduction with parentheses runs on tidal-fe.
struct S {
  int v;
  S(int x) : v(x) {}
  template <class T> S(T* p) : v(*p) {}
};
int main() { S s(5); return s.v - 5; }
