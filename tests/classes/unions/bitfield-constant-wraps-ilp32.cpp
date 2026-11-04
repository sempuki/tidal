// tidal-rules: BF-10
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32. The same choice as on x86-64 (bitfield-constant-wraps.cpp). A value that does not fit takes the value of its range that differs by a multiple of 2^n.
struct S { unsigned x : 3; int y : 3; };
constexpr S make() { S s{}; s.x = 9; s.y = 5; return s; }
static_assert(make().x == 1);
static_assert(make().y == -3);
