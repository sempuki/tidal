// tidal-rules: CAST-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct E { explicit E(int) {} };
struct C { explicit operator int() const { return 1; } };
E e = static_cast<E>(3);
int i = static_cast<int>(C{});
