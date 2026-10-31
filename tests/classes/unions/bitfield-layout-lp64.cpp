// tidal-rules: BF-9
// tidal-expect: accept
struct S { unsigned a : 3; unsigned b : 5; unsigned c : 8; };
static_assert(sizeof(S) == sizeof(unsigned));
struct T { char a : 4; char b : 4; };
static_assert(sizeof(T) == 1);
