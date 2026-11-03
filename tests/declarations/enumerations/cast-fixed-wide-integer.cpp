// tidal-rules: ENUM-21
// tidal-expect: accept
// tidal-note: Integers of type long long convert to enumerations with unsigned short and signed short as underlying types.
enum Port : unsigned short { none };
static_assert(static_cast<long long>(static_cast<Port>(70000LL)) == 4464);
enum class Delta : short { zero };
static_assert(static_cast<int>(static_cast<Delta>(40000LL)) == -25536);
