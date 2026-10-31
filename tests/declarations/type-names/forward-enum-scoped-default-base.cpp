// tidal-rules: TYN-6
// tidal-expect: accept
enum class Scoped;
Scoped* p = nullptr;
static_assert(sizeof(Scoped) == sizeof(int));
