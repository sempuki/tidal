// tidal-rules: PRIM-7
// tidal-expect: accept
static_assert(sizeof(R"(a\n)") == 4);
static_assert(sizeof(R"x(a)"b)x") == 5);
static_assert(R"(\)"[0] == 92);
