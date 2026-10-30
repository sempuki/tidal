// tidal-rules: NS-7
// tidal-expect: run
namespace N { int f(int) { return 1; } int f(double) { return 2; } }
using N::f;
int main() { return f(1) == 1 && f(1.5) == 2 ? 0 : 1; }
