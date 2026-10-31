// tidal-rules: PRIM-12
// tidal-expect: run
struct S { static int count; enum { size = 4 }; };
int S::count = 3;
int main() { return S::count + S::size == 7 ? 0 : 1; }
