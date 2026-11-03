// tidal-rules: TYN-6
// tidal-expect: accept
// tidal-note: A scoped enumeration declared without a type has the type int, so the two declarations agree.
enum class E : int;
enum class E { a };
enum class F;
enum class F : int { b };
static_assert(sizeof(E) == sizeof(int) && sizeof(F) == sizeof(int));
