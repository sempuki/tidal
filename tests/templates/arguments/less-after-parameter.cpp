// tidal-rules: TARG-2
// tidal-expect: accept
// tidal-note: The function parameter Size hides the class template, so the call has two arguments, Size < 3 and 4 > (0). Where Size names the template, Size<3, 4> is one type.
template <int A, int B> struct Size {};
constexpr int count(bool, bool) { return 2; }
constexpr int count(Size<3, 4>) { return 1; }
constexpr int use(int Size) { return count(Size < 3, 4 > (0)); }
static_assert(use(1) == 2);
static_assert(count(Size<3, 4>{}) == 1);
