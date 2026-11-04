// tidal-rules: LSP-4
// tidal-expect: run
// tidal-expect-gcc: reject
// tidal-expect-gcc-16-o2: run
// tidal-error-gcc: symbol .cb. is already defined
// tidal-note: Q-0369. A static C function belongs to its own namespace. GCC accepts the source, but its assembler sees one symbol defined twice. GCC 16 at -O2 emits no symbol for the functions, and the program runs.
namespace a { extern "C" { static int cb() { return 1; } } int first() { return cb(); } }
namespace b { extern "C" { static int cb() { return 2; } } int second() { return cb(); } }
int main() { return a::first() == 1 && b::second() == 2 ? 0 : 1; }
