// tidal-rules: DTY-3
// tidal-expect: accept
struct B { int x; };
B b;
struct D : decltype(b) {};
int get(D d) { return d.x; }
