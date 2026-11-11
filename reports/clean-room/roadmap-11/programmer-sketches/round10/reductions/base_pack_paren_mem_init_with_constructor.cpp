// The same pack of bases built in a complete object. The front end rejects with CTOR-15.
template <int I> struct Leaf {
  int value;
  Leaf() = default;
  Leaf(int v) : value(v) {}
};
template <int... I> struct Impl : Leaf<I>... {
  Impl(int v) : Leaf<I>(v + I)... {}
};
int main() {
  Impl<0, 1> t(5);
  Leaf<0>& first = t;
  Leaf<1>& second = t;
  return (first.value == 5 && second.value == 6) ? 0 : 1;
}
