template <int D> struct Q {
  long long value{};
  constexpr explicit Q(long long v) : value(v) {}
  template <class U = long long>
    requires (D == 0)
  constexpr explicit operator U() const { return value; }
};
template <int A, int B> constexpr Q<A - B> operator/(Q<A> a, Q<B> b) { return Q<A - B>(a.value / b.value); }
static_assert((Q<1>(10) / Q<2>(2)).value == 5);
int main() { return 0; }
