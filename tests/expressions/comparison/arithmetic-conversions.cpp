// tidal-rules: CMP-2
// tidal-expect: accept
static_assert(1 == 1.0);
static_assert('a' < 98);
static_assert(2.5f > 2);
enum E { A = 3 };
static_assert(A == 3 && A > 2.5);
static_assert(true == 1);
