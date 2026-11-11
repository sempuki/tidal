template <class K> auto scale(K k) {
  return [k](int r) { return r * k; };
}
int main() { return scale(2)(3) == 6 ? 0 : 1; }
