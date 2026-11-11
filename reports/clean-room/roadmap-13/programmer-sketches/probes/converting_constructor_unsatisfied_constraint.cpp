template <bool C> struct It {
  int v = 0;
  It() = default;
  It(int x) requires C : v(x) {}
};
int main() { It<false> b = 5; return b.v; }
