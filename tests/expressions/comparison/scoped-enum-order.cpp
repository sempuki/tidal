// tidal-rules: CMP-10
// tidal-expect: accept
enum class Level : unsigned char { low = 1, mid = 5, high = 9 };
static_assert(Level::low < Level::mid);
static_assert(Level::high >= Level::mid);
static_assert(Level::mid == Level::mid);
static_assert(Level::low != Level::high);
static_assert(!(Level::high <= Level::low));
