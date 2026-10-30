// tidal-rules: UN-16
// tidal-expect: accept
static_assert(alignof(char) == 1);
static_assert(alignof(int) >= 1 && (alignof(int) & (alignof(int) - 1)) == 0);
static_assert(alignof(double) % alignof(char) == 0);
struct S { char c; int i; };
static_assert(alignof(S) >= alignof(int));
static_assert(alignof(int[4]) == alignof(int));
static_assert(alignof(int&) == alignof(int));
