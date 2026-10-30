// tidal-rules: NS-7
// tidal-expect: run
namespace N { int f(int) { return 1; } }
using N::f;
namespace N { int f(double) { return 2; } }
int main() { return f(1.5) == 1 ? 0 : 1; }
