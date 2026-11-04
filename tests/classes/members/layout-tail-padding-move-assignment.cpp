// tidal-rules: CLS-35 CLS-34
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Q-0361. A base with a user-provided move assignment operator keeps its end padding on GCC and Clang, and the same holds for a marked member. EDG lends the padding in both places.
struct Moved { Moved& operator=(Moved&&) { return *this; } int a; char b; };
struct Next : Moved { char c; };
struct Holder { [[no_unique_address]] Moved part; char c; };
static_assert(sizeof(Next) == 12);
static_assert(sizeof(Holder) == 12);
