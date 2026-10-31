// tidal-rules: TYN-1
// tidal-expect: reject
using Int = int;
void f(int);
void f(Int) {}
void f(int) {}
