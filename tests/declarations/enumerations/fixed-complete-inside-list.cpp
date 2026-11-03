// tidal-rules: ENUM-10
// tidal-expect: accept
enum class Size : int { word = sizeof(Size) };    // complete: the type is fixed
enum Small : short { bytes = sizeof(Small) };
static_assert(static_cast<int>(Size::word) == sizeof(int));
static_assert(bytes == sizeof(short));
