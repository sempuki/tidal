// Probe: a constructor template with the wrong number of parameters
// does not stop braced initialization on tidal-fe.
struct S {
  int v;
  S(int x) : v(x) {}
  template <class T> S(T, T) : v(1) {}
};
int main() { S s{0}; return s.v; }
