// tidal-rules: TYN-7
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Q-0093. GCC accepts a declaration of only a class key and a qualified name. Clang and EDG reject it.
namespace n { struct S; }
struct n::S;
