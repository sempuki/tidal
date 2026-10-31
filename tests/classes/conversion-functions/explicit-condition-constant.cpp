// tidal-rules: CVF-3
// tidal-expect: accept
struct S { explicit(true) operator int() const { return 1; } };
struct T { explicit(false) operator int() const { return 1; } };
int x = T{};
int y(S{});
