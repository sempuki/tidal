struct M {
  template <class V> M(V v) : value(v) {}
  int value;
};
int main() { M y{2}; return y.value == 2 ? 0 : 1; }
