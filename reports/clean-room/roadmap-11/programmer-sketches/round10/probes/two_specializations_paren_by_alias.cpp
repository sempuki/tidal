// The same with each base named through a type alias, in parentheses.
template <int I, class T> struct Leaf {
  T value;
  Leaf(T v) : value(v) {}
};
using L0 = Leaf<0, int>;
using L1 = Leaf<1, int>;
struct Pair : L0, L1 {
  Pair(int a, int b) : L0(a), L1(b) {}
};
int main() {
  Pair p(1, 2);
  return (p.L0::value == 1 && p.L1::value == 2) ? 0 : 1;
}
