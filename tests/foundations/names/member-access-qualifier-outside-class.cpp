// tidal-rules: NAM-37
// tidal-expect: accept
// tidal-note: The qualifier Alias is not a member of D. It is looked up from the point of the use and names the base B.
struct B { int m; };
struct D : B {};
using Alias = B;
int f(D d) { return d.Alias::m; }
