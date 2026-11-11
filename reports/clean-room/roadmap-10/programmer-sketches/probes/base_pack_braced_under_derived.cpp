// The tested braced form (aggregate leaves, leaf<I>{v + I}...) inside a
// class that a derived class builds.
template <int I> struct Leaf { int value; };
template <int... I> struct Impl : Leaf<I>... {
  Impl(int v) : Leaf<I>{v + I}... {}
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
