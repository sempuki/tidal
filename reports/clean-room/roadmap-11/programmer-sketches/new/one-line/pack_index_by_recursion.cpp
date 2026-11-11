// The Nth element of a pack by recursion and by a fold over the comma operator.
template <int N, class T, class... Ts> constexpr auto nth(T t, Ts... ts) { if constexpr (N == 0) return t; else return nth<N - 1>(ts...); }
template <class... Ts> constexpr int last(Ts... ts) { int r = 0; ((r = ts), ...); return r; }
int main() { return nth<2>(1, 2.5, 'c') == 'c' && last(4, 5, 6) == 6 ? 0 : 1; }
