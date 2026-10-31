// tidal-rules: ENUM-21
// tidal-expect: accept
enum Byte : unsigned char { zero };
static_assert(static_cast<int>(static_cast<Byte>(300)) == 44);
enum Small : signed char { none };
static_assert(static_cast<int>(static_cast<Small>(200)) == -56);
enum class Wide : unsigned char { first };
static_assert(static_cast<int>(static_cast<Wide>(-1)) == 255);
enum Flag : bool { off, on };
static_assert(static_cast<Flag>(5) == on);
