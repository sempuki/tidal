// tidal-rules: TPL-37
// tidal-expect: reject
// tidal-error-clang: default template argument not permitted on a friend template
// tidal-error-gcc: default template arguments may not be used in template friend declarations
// tidal-error-edg: a default template argument in a friend
struct Host { template <class T = int> friend int pick(Host); };
