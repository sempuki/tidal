// tidal-rules: INST-21
// tidal-expect: reject
// tidal-error-clang: must occur in namespace 'lib'
// tidal-error-gcc: outside its namespace must use a nested-name-specifier
// tidal-error-edg: is not a class or function template name
// tidal-note: The explicit instantiation names the function template get without lib::, outside namespace lib. Every compiler rejects it. EDG accepts the same form for a class or a variable template (Q-0235).
namespace lib { template <class T> int get(T) { return 1; } }
using namespace lib;
template int get(int);
