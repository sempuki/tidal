// tidal-rules: STG-1
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-error-gcc: overflow in constant expression
// tidal-error-clang: is too large
// tidal-error-edg: class is too large
// tidal-note: Clang 18 accepts the class and gives sizeof a value reduced modulo the range of the size type (Q-0308).
struct Block { char bytes[1ull << 59]; };
struct Many {
  Block a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p;
  Block q, r, s, t, u, v, w, x, y, z, aa, bb, cc, dd, ee, ff, last;
};
constexpr auto size = sizeof(Many);
