// tidal-rules: ODR-3 ODR-9 ODR-11
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-flags: -Werror
// tidal-error-clang: has internal linkage but is not defined
// tidal-note: Q-0315. The template argument &hidden uses hidden (ODR-3), and hidden has internal linkage and no definition. ODR-9 needs no diagnostic. Clang reports the use under ODR-11, and GCC gives no diagnostic.
template <int (*F)()> struct Holder { static int get() { return 0; } };
static int hidden();
int main() { return Holder<&hidden>::get(); }
