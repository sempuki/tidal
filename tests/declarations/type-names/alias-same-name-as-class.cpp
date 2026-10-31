// tidal-rules: TYN-1
// tidal-expect: accept
struct S { int v; };
typedef struct S S;
using S = S;
S s{1};
