// tidal-rules: ATTR-20
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-expect-gcc: reject
// tidal-error-clang: static assertion failed
// tidal-error-gcc: static assertion failed
// tidal-note: The effect of a vendor attribute is implementation-defined (ATTR-20). EDG 7.0 in strict mode keeps the usual layout. Clang and GCC apply gnu::packed (vendor-attribute-changes-layout.cpp).
struct [[gnu::packed]] Record { char kind; int value; };
static_assert(sizeof(Record) == 2 * sizeof(int));   // padding before value
