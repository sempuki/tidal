// tidal-rules: PFX-15 PFX-22
// tidal-expect: accept
// tidal-note: Gap review of the text phase 8 wrote on expressions. Passing a scoped enumeration to an ellipsis parameter is conditionally supported, and the argument keeps its type. Clang 22, GCC 16, and EDG 7.0 all support it.
enum class Level : int { low, high };
int take(int count, ...);
int use() { return take(1, Level::high); }
