template <int I> struct leaf { int value; };

template <int... I>
struct all : leaf<I>... {
  all(int v) : leaf<I>{v}... {}
};

int main() {
  all<0> a(5);
  return a.value == 5 ? 0 : 1;
}
