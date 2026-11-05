// tidal-rules: NAM-21
// tidal-expect: accept
// tidal-note: Gap review of the text phase 8 wrote on expressions, foundations, and templates. The enumerator list of an enumeration defined with a qualified name sees the members of the namespace without qualification. Clang 22, GCC 16, and EDG 7.0 accept.
namespace lib {
  constexpr int start = 4;
  enum class Level : int;
  enum Plain : int;
}
enum class lib::Level : int { low = start, high };
enum lib::Plain : int { first = start + 1 };
static_assert(static_cast<int>(lib::Level::high) == 5);
static_assert(lib::first == 5);
