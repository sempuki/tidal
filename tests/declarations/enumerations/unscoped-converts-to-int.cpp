// tidal-rules: ENUM-2
// tidal-expect: accept
enum E { A = 3 };
int i = A;
static_assert(A + 1 == 4);
static_assert(A < 5);
