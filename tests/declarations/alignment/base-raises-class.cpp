// tidal-rules: ALN-1
// tidal-expect: accept
struct Base { alignas(16) char c; };
struct Derived : Base { char x; };
static_assert(alignof(Derived) == 16);
static_assert(sizeof(Derived) == 32);
struct alignas(32) Tagged {};
struct Holder : Tagged { char x; };
static_assert(alignof(Holder) == 32);
