template <int D> struct Q {
  static constexpr int dim = D;
  long long value{};
  constexpr explicit Q(long long v) : value(v) {}
  template <class U = long long>
    requires (D == 0)
  constexpr explicit operator U() const { return value; }
};
template <int A, int B> constexpr Q<A - B> operator/(Q<A> a, Q<B> b) { return Q<A - B>(a.value / b.value); }
static_assert(decltype(Q<1>(10) / Q<2>(2))::dim == -1);
int main() { return 0; }
