// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-flags: -m32
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: Q-0381. -m32. The sizes are the same as on x86-64 (layout-no-unique-address-end-padding.cpp). GCC lends the end padding of a class with a [[no_unique_address]] member.
struct Empty {};
struct WithEmpty { int i; char c; [[no_unique_address]] Empty e; };
struct WithChar { int i; [[no_unique_address]] char c; };
struct AddEmpty : WithEmpty { char z; };
struct AddChar : WithChar { char z; };
static_assert(sizeof(WithEmpty) == 8 && sizeof(WithChar) == 8);
static_assert(sizeof(AddEmpty) == 12);
static_assert(sizeof(AddChar) == 12);
