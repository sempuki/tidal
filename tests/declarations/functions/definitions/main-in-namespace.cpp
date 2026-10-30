// tidal-rules: FN-4
// tidal-expect: run
namespace N { int main() { return 1; } }
int main() { return N::main() - 1; }
