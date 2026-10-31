// tidal-rules: NAM-4
// tidal-expect: run
namespace n { int a = 1; }
namespace n { int b = a + 1; }
int main() { return n::b == 2 ? 0 : 1; }
