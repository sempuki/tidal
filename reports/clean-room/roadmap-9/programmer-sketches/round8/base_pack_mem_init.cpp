template <int I> struct leaf { int value; };

template <int... I>
struct all : leaf<I>... {
  all(int v) : leaf<I>{v + I}... {}
};

int main() {
  all<0, 1> a(5);
  return a.leaf<1>::value - a.leaf<0>::value == 1 ? 0 : 1;
}
