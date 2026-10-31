// tidal-rules: DTOR-3
// tidal-expect: accept
struct S { int v; ~S() = delete; };
S make();
using Made = decltype(make());
Made* p = new Made{1};
