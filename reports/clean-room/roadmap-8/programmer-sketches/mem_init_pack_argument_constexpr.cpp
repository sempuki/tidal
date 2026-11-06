struct base {
  int n;
  constexpr base(int a, int b) : n(a + b) {}
};
template <class S> struct derived;
template <class... T>
struct derived<void(T...)> : base {
  constexpr derived(T... v) : base(v...) {}
};
int main() {
  derived<void(int, int)> d(2, 3);
  return d.n == 5 ? 0 : 1;
}
