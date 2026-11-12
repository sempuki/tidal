template <class S> struct Scal {
  template <class N>
    requires requires(const S& s, N n) { s.get() * n; }
  friend constexpr S operator*(const S& s, N n) { return S(s.get() * n); }
  template <class N>
    requires requires(const S& s, N n) { s.get() * n; }
  friend constexpr S operator*(N n, const S& s) { return S(s.get() * n); }
};
struct M : Scal<M> {
  long long v;
  constexpr explicit M(long long x) : v(x) {}
  constexpr long long get() const { return v; }
};
constexpr M c = 2 * M(30) * 1;
int main() { return c.v == 60 ? 0 : 1; }
