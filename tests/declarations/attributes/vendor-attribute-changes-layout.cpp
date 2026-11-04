// tidal-rules: ATTR-20
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: The effect of a vendor attribute is implementation-defined (ATTR-20). Clang and GCC apply gnu::packed. EDG 7.0 in strict mode does not (vendor-attribute-not-applied.cpp).
struct [[gnu::packed]] Record { char kind; int value; };
static_assert(sizeof(Record) == 5);   // no padding before value
