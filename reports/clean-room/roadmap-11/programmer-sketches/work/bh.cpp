template <class K> auto scale(K k) {
  return [](auto r) { return r * 2; };
}
int main() { return scale(2)(3) == 6 ? 0 : 1; }
