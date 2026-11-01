// tidal-rules: PACK-14
// tidal-expect: accept
// tidal-note: Bs... gives one base per element, in order, and Bs(v)... gives one member initializer per base.
struct Left { int left; constexpr Left(int v) : left(v) {} };
struct Right { int right; constexpr Right(int v) : right(v) {} };
template <class... Bs> struct Same : Bs... { constexpr Same(int v) : Bs(v)... {} };
constexpr Same<Left, Right> same(4);
static_assert(same.left == 4 && same.right == 4);
template <class... Bs> struct Each : Bs... { constexpr Each(Bs... b) : Bs(b)... {} };
constexpr Each<Left, Right> each(Left(1), Right(2));
static_assert(each.left == 1 && each.right == 2);
