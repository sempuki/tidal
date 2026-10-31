// tidal-rules: CVF-3
// tidal-expect: accept
struct S { explicit operator int() const { return 1; } };
int x(S{});
int y = static_cast<int>(S{});
int z = int(S{});
