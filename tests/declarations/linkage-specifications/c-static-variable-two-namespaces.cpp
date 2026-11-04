// tidal-rules: LSP-4
// tidal-expect: run
// tidal-expect-gcc: reject
// tidal-expect-gcc-14: run
// tidal-expect-gcc-16-o2: run
// tidal-error-gcc: symbol .count. is already defined
// tidal-note: Q-0369. A static C variable belongs to its own namespace. GCC accepts the source, but its assembler sees one symbol defined twice. GCC 14, and GCC 16 at -O2, emit no symbol for the variables, and the program runs.
namespace a { extern "C" { static int count = 1; } int first() { return count; } }
namespace b { extern "C" { static int count = 2; } int second() { return count; } }
int main() { return a::first() == 1 && b::second() == 2 ? 0 : 1; }
