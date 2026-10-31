// tidal-rules: TYN-6
// tidal-expect: run
struct Outer { struct Inner; Inner* p = nullptr; };
struct Outer::Inner { int v = 2; };
int main() { Outer::Inner i; Outer o; o.p = &i; return o.p->v == 2 ? 0 : 1; }
