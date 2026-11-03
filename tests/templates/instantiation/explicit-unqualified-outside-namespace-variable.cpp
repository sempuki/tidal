// tidal-rules: INST-21
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-edg: accept
// tidal-error-gcc: outside its namespace must use a nested-name-specifier
// tidal-note: The explicit instantiation names the variable template value without lib::, outside namespace lib. GCC rejects it, Clang and EDG accept it (Q-0235).
namespace lib { template <class T> int value = 1; }
using namespace lib;
template int value<int>;
