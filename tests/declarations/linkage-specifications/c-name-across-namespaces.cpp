// tidal-rules: LSP-4
// tidal-expect: run
namespace a { extern "C" int shared(); }
namespace b { extern "C" int shared(); }
extern "C" int shared() { return 7; }
int main() { return a::shared() + b::shared() == 14 ? 0 : 1; }
