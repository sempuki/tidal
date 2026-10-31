// tidal-rules: ILC-1
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .P
// tidal-error-gcc: no matching function for call to .P::P
// tidal-error-edg: no instance of constructor .P::P
struct P { int x, y; P() {} };
P p{1, 2};
