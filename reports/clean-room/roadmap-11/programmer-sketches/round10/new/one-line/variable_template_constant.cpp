// A variable template for a constant at several types, and a specialization.
template <class T> constexpr T pi = T(3.1415926535897932385L);
template <> constexpr int pi<int> = 3;
int main() {
  static_assert(pi<int> == 3);
  double d = pi<double>;
  float f = pi<float>;
  return d > 3.14 && d < 3.15 && f > 3.14f ? 0 : 1;
}
