constexpr auto scale(auto k) {
  return [k](auto r) { return r * k; };
}
static_assert(scale(2)(3) == 6);
int main() {}
