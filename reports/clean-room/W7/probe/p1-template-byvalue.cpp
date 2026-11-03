// Probe: a constructor template that could instantiate as T(T).
template <int N> struct M {
  int v;
  constexpr M(int x) : v(x) {}
  template <int K> constexpr M(M<K> o) : v(o.v + 1000) {}
};
constexpr M<1> a(5);
constexpr M<1> b(a);            // copy constructor or template with K = 1?
constexpr M<1> c(M<2>(5));
static_assert(b.v == 5);
static_assert(c.v == 1005);
int main() { return 0; }
