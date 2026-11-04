// tidal-rules: LSP-4
// tidal-expect: run
// tidal-note: Q-0369.
namespace { extern "C" int code() { return 3; } }
namespace b { extern "C" int code(); }     // the same function
int main() { return b::code() == 3 ? 0 : 1; }
