// tidal-rules: TPL-7
// tidal-expect: reject
// tidal-error-clang: non-literal type
// tidal-error-gcc: not structural
// tidal-error-edg: structural class type
// tidal-note: Point is not an aggregate and has no constexpr constructor, so it is not a literal type. Its member is public and of type int.
struct Point { int v; Point(int x) : v(x) {} };
template <Point P> struct Use {};
