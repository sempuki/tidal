// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: Q-0381. x86-64. Clang and EDG keep the end padding of a class that meets the conditions of CLS-35 and has a [[no_unique_address]] member. GCC lends the padding of such a class, so each derived class below has size 8 on GCC.
struct Empty {};
// i 0, c 4, e 0, data size 5, size 8
struct WithEmpty { int i; char c; [[no_unique_address]] Empty e; };
struct WithChar { int i; [[no_unique_address]] char c; };
struct AddEmpty : WithEmpty { char z; };
struct AddChar : WithChar { char z; };
static_assert(sizeof(WithEmpty) == 8 && sizeof(WithChar) == 8);
static_assert(sizeof(AddEmpty) == 12);
static_assert(sizeof(AddChar) == 12);
