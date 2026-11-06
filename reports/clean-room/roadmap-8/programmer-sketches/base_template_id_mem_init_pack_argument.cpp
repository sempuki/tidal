template <class... T> struct base {
  int n;
  base(T... v) : n((v + ...)) {}
};

template <class... T>
struct derived : base<T...> {
  derived(T... v) : base<T...>(v...) {}
};

int main() {
  derived<int, int> d(2, 3);
  return d.n == 5 ? 0 : 1;
}
