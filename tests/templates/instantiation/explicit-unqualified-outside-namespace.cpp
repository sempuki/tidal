// tidal-rules: INST-21
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: The explicit instantiation names Box without lib::, outside namespace lib. Clang and GCC reject it, EDG accepts it (Q-0235).
// tidal-error-clang: must occur in namespace 'lib'
// tidal-error-gcc: outside its namespace must use a nested-name-specifier
namespace lib { template <class T> struct Box {}; }
using namespace lib;
template struct Box<int>;
