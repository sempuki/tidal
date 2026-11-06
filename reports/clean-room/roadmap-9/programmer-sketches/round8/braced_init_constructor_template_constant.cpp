struct M {
  template <class V> constexpr M(V v) : value(v) {}
  int value;
};
constexpr M x{1};
static_assert(x.value == 1);
int main() { M y{2}; return y.value == 2 ? 0 : 1; }
