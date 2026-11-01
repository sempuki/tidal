// tidal-rules: DED-17
// tidal-expect: accept
// tidal-note: Each element is an argument for T. In {1, 2L} the elements give int and long, so deduction fails and the plain function is called. In {1.5, 2.5} T is double and N is 2.
template <class T, int N> constexpr int len(const T (&)[N]) { return N; }
constexpr int len(const long (&)[2]) { return 20; }
static_assert(len({1.5, 2.5}) == 2);
static_assert(len({1, 2L}) == 20);
template <class T, int M, int N> constexpr int area(const T (&)[M][N]) { return M * 10 + N; }
static_assert(area({{1, 2, 3}}) == 13);
