// tidal-rules: LIST-8
// tidal-expect: accept
struct S { int a; double b; int* p; };
constexpr S s = {1};
static_assert(s.a == 1 && s.b == 0.0 && s.p == nullptr);
