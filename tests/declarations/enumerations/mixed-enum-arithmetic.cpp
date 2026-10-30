// tidal-rules: ENUM-2
// tidal-expect: accept
enum E { A = 1 };
enum F { B = 2 };
static_assert(A + B == 3);
