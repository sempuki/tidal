// tidal-rules: OPR-5
// tidal-expect: run
struct Target { int v = 6; };
struct Inner { Target t; Target* operator->() { return &t; } };
struct Outer { Inner i; Inner& operator->() { return i; } };
int main() { Outer o; return o->v == 6 ? 0 : 1; }
