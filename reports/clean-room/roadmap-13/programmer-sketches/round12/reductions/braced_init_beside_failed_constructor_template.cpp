// From ring_fifth_try.cpp. In list-initialization, a constructor
// template whose deduction fails adds no candidate (DED-35, ILC-3), and
// S(int) is chosen. tidal-fe stops with "constructor call". S s(5) runs.
struct S {
  int v;
  S(int x) : v(x) {}
  template <class T> S(T* p) : v(*p) {}
};
int main() { S s{5}; return s.v - 5; }
