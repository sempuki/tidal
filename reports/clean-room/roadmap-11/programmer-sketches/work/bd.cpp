constexpr auto scale(int k) {
  return [k](auto r) { return r * k; };
}
static_assert(scale(2)(3) == 6);
int main() { return scale(2)(3) == 6 ? 0 : 1; }
