// A lambda with a template head and a requires clause.
template <class T> concept Integral = T(1) / T(2) == T(0);
int main() {
  auto half = []<class T> requires Integral<T> (T x) { return x / 2; };
  return half(9) - 4 + (half(9L) - 4);
}
