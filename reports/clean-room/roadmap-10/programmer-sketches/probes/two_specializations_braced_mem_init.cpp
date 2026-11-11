// two_specializations_of_one_base_template_mem_init.cpp with braces.
template <int I, class T> struct Leaf {
  T value;
  Leaf(T v) : value(v) {}
};
struct Pair : Leaf<0, int>, Leaf<1, int> {
  Pair(int a, int b) : Leaf<0, int>{a}, Leaf<1, int>{b} {}
};
int main() {
  Pair p(1, 2);
  return (p.Leaf<0, int>::value == 1 && p.Leaf<1, int>::value == 2) ? 0 : 1;
}
