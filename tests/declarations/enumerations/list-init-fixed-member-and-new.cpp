// tidal-rules: ENUM-8
// tidal-expect: accept
// tidal-note: Direct-list-initialization from an integer in a member initializer and in a new expression.
enum class Level : unsigned char { low };
struct Holder {
  Level level;
  constexpr Holder() : level{2} {}
};
static_assert(static_cast<int>(Holder{}.level) == 2);
Level* make() { return new Level{4}; }
