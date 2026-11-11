template <class K> constexpr auto scale(K k) {
  return [k](auto r) { return r * k; };
}
constexpr int v = scale(2)(3);
int main() { return v == 6 ? 0 : 1; }
