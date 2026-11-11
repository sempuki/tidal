template <class K> constexpr auto scale(K k) {
  return [k](auto r) { return r * k; };
}
int main() { constexpr auto f = scale(2); return f(3) == 6 ? 0 : 1; }
