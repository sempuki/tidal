// tidal-rules: LSP-4 ODR-9
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-error-clang: has internal linkage but is not defined
// tidal-error-gcc: used but never defined
// tidal-error-edg: was referenced but not defined
// tidal-note: Q-0369. The static C function in b is a new function, not the one in a, and it has no definition. ODR-9 needs no diagnostic, and Clang 18 only warns.
namespace a { extern "C" { static int cb() { return 1; } } }
namespace b { extern "C" { static int cb(); } }
int main() { return b::cb(); }
