// tidal-rules: LSP-1
// tidal-expect: run
namespace lib { extern "C" int answer() { return 42; } }
int main() { return lib::answer() == 42 ? 0 : 1; }
