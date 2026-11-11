// A pack of bases with constructors, built by a mem-initializer pack, inside a
// class that a derived class builds. The front end drops the initializers.
template <int I> struct Leaf {
  int value;
  Leaf() = default;
  Leaf(int v) : value(v) {}
};
template <int... I> struct Impl : Leaf<I>... {
  Impl(int v) : Leaf<I>(v + I)... {}
};
struct Top : Impl<0, 1> {
  Top(int v) : Impl<0, 1>(v) {}
};
int main() {
  Top t(5);
  Leaf<0>& first = t;
  Leaf<1>& second = t;
  return (first.value == 5 && second.value == 6) ? 0 : 1;
}
