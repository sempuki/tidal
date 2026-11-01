// tidal-rules: TPL-37
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: default template argument not permitted on a friend template
// tidal-error-edg: default argument is not allowed on a friend
// tidal-note: GCC accepts the default on a friend class template declaration (Q-0196).
template <class T> struct Box {};
struct Host { template <class T = int> friend struct Box; };
