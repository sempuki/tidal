// tidal-rules: LAM-11
// tidal-expect: accept
auto make = [] { return 7; };
using Make = decltype(make);
Make copy;
void f() { Make other; other = copy; }
