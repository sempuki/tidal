template <bool C> struct It {
  int v = 0;
  It() = default;
  It(const It<false>& o) requires C : v(o.v + 1) {}
};
int main() {
  It<false> a;
  It<false> b = a;
  return b.v;
}
