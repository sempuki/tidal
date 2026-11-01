// tidal-rules: ALN-8
// tidal-expect: accept
// tidal-note: An enumerator of an unscoped enumeration is a valid operand.
enum Width { eight = 8 };
struct alignas(eight) Block { int v; };
static_assert(alignof(Block) == 8);
