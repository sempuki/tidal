template <class K> constexpr auto scale(K k) {
  return [k](auto r) { return r * k; };
}
static_assert(scale(2)(3) == 6);
int main() {}
