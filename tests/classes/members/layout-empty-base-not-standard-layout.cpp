// tidal-rules: CLS-31 CLS-33 CLS-10
// tidal-expect: accept
// tidal-note: Both is empty and not standard-layout, because it has two Unit subobjects. Its bases Left and Right are empty and standard-layout, so they take no bytes (CLS-33). Its two Unit subobjects still get two addresses. As a base, Both takes no bytes on the matrix targets (CLS-31).
struct Unit {};
struct Left : Unit {};
struct Right : Unit {};
struct Both : Left, Right {};
struct User : Both { int value; };
static_assert(sizeof(Both) == 2);
static_assert(sizeof(User) == sizeof(int));
