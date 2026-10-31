// tidal-rules: NAM-4
// tidal-expect: run
namespace n { int f(); }
int n::f() { return 4; }
int main() { return n::f() == 4 ? 0 : 1; }
