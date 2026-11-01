// tidal-rules: TSP-19
// tidal-expect: accept
// tidal-note: An explicit specialization in the global namespace with a qualified name, for a template in lib.
namespace lib { template <class T> struct Box { static constexpr int tag = 0; }; }
template <> struct lib::Box<int> { static constexpr int tag = 1; };
static_assert(lib::Box<int>::tag == 1);
static_assert(lib::Box<char>::tag == 0);
