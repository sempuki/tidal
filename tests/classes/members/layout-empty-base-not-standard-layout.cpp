// tidal-rules: CLS-31 CLS-10
// tidal-expect: accept
// tidal-note: Both is empty and not standard-layout, because it has two Unit subobjects. As a base, Both takes no bytes on the matrix targets. Its two Unit subobjects still get two addresses.
struct Unit {};
struct Left : Unit {};
struct Right : Unit {};
struct Both : Left, Right {};
struct User : Both { int value; };
static_assert(sizeof(Both) == 2);
static_assert(sizeof(User) == sizeof(int));
