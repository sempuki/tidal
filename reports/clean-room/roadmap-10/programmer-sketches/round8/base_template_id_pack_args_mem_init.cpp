template <class... T> struct base {
  int n = sizeof...(T);
};

template <class... T>
struct derived : base<T...> {
  derived() : base<T...>() {}
};

int main() {
  derived<int, char> d;
  return d.n == 2 ? 0 : 1;
}
