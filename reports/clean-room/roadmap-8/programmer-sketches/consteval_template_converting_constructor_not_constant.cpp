void fail();

template <class... A>
struct checked {
  int n;
  consteval checked(int v) : n(v) { if (v != sizeof...(A)) fail(); }
};

template <class T> struct id { using type = T; };

template <class... A>
int take(checked<typename id<A>::type...> c, A...) { return c.n; }

int main() { return take(2, 1); }
