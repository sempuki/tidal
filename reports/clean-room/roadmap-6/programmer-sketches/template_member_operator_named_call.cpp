template <int N> struct Fixed {
  long long raw;
  bool operator<(Fixed o) const;
};
template <int N> bool Fixed<N>::operator<(Fixed o) const { return raw < o.raw; }
static_assert(true);
constexpr bool f() { return false; }
int main() {
  Fixed<1> a{3}, b{5};
  return a.operator<(b) ? 0 : 1;
}
