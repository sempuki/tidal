// tidal-rules: PRIM-12
// tidal-expect: run
int v = 1;
namespace n { int v = 2; int get() { return v * 10 + ::v; } }
int main() { return n::get() == 21 ? 0 : 1; }
