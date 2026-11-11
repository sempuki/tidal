template <class K> constexpr auto scale(K k) {
  return [k](int r) { return r * k; };
}
static_assert(scale(2)(3) == 6);
int main() {}
