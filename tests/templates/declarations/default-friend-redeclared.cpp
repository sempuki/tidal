// tidal-rules: TPL-38
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-gcc: accept
// tidal-error-edg: a friend template declaration with a default
// tidal-note: The friend definition with a default is followed by another declaration of the function template, which breaks TPL-38. EDG rejects it, and Clang and GCC accept it (Q-0197).
struct Host {
  template <class T = int> friend int pick(Host) { return sizeof(T); }
};
template <class T> int pick(Host);
