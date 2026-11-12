struct P { int x, y; };
template <class T, class F> void each(const T& t, F f) {
  if constexpr (sizeof(T) == sizeof(int)) {
    auto& [a] = t;
    f(a);
  } else {
    auto& [a, b] = t;
    f(a); f(b);
  }
}
int main() { P p{1, 2}; int s = 0; each(p, [&](int v) { s += v; }); return s - 3; }
