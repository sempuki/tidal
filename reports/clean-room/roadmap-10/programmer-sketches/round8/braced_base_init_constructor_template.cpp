struct M {
  template <class V> constexpr M(V v) : value(v) {}
  int value;
};
struct D : M {
  D(int v) : M{v} {}
};
int main() { D d(2); return d.value == 2 ? 0 : 1; }
