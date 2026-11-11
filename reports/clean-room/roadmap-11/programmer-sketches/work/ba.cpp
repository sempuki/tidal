template <class F> struct Adaptor { F make; };
constexpr auto scale(auto k) {
  return Adaptor([k](auto r) { return r * k; });
}
static_assert(scale(2).make(3) == 6);
int main() {}
