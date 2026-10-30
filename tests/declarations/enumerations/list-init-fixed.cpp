// tidal-rules: ENUM-8
// tidal-expect: accept
enum class F { A };
enum G : int { B };
F f{5};
G g{6};
constexpr F h{3};
static_assert(static_cast<int>(h) == 3);
