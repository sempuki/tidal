// tidal-rules: CVF-1
// tidal-expect: accept
struct S { int v; constexpr operator int() const { return v; } };
constexpr S s{6};
static_assert(s == 6);
int arr[S{2}];
