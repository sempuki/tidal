// tidal-rules: NEW-27
// tidal-expect: accept
static_assert(__STDCPP_DEFAULT_NEW_ALIGNMENT__ == 16);
struct alignas(__STDCPP_DEFAULT_NEW_ALIGNMENT__) Block { char c; };
static_assert(alignof(Block) == 16);
