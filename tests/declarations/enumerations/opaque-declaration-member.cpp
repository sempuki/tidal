// tidal-rules: ENUM-7
// tidal-expect: accept
// tidal-note: An opaque declaration in a class, used in a function declaration, and the definition after the class.
struct S {
  enum class Kind : short;
  static int code(Kind k);
};
enum class S::Kind : short { low = 1, high = 9 };
constexpr S::Kind k = S::Kind::high;
static_assert(static_cast<int>(k) == 9);
