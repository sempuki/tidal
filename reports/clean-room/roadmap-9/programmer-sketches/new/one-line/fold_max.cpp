template <class T, class... U> constexpr T vmax(T first, U... rest) { T m = first; ((m = rest > m ? rest : m), ...); return m; }
static_assert(vmax(3, 9, 2, 7) == 9);
int main() { return vmax(1) == 1 ? 0 : 1; }
