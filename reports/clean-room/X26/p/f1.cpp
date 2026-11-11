// Folds over the comma. -DTWO instantiates the value fold with two elements.
int price(int v) { return v * 2; }
template <class... T> int last(T... t) { return price((t, ...)); }
template <class... T> void names(T... t) { (t, ...); }
int r = 0;
template <int... V> void assign() { ((r = r * 10 + V), ...); }
int main() {
  names();
  assign<1, 2>();
#ifdef TWO
  return last(1, 2) == 4 && r == 12 ? 0 : 1;
#else
  return last(1) == 2 && r == 12 ? 0 : 1;
#endif
}
