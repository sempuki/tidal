// tidal-rules: REF-11
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: reference cannot be initialized with multiple values
// tidal-error-edg: too many initializer values
// tidal-note: Q-0151. GCC initializes a temporary aggregate from the parenthesized values.
struct Point { int x, y; };
const Point& p(1, 2);
