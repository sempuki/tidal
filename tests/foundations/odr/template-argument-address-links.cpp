// tidal-rules: ODR-8 ODR-3
// tidal-expect: run
// tidal-note: Q-0315. The template argument &declared_only uses declared_only, which has no definition. Nothing calls it, so no compiler emits a reference to it, and the program links. ODR-8 needs no diagnostic and allows the result.
template <int (*F)()> struct Holder { static int get() { return 0; } };
int declared_only();
int main() { return Holder<&declared_only>::get(); }
