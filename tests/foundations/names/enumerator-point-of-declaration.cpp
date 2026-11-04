// tidal-rules: NAM-1
// tidal-expect: accept
// tidal-note: The point of declaration of an enumerator comes after its initializer. The initializer x + 1 sees the outer constant x.
constexpr int x = 5;
void f() { enum { x = x + 1 }; static_assert(x == 6); }
