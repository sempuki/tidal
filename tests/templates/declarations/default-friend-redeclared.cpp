// tidal-rules: TPL-38
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: a friend template declaration with a default
// tidal-note: The friend definition with a default is followed by another declaration of the function template. Clang and GCC accept, EDG rejects, and TPL-38 allows both (Q-0197).
struct Host {
  template <class T = int> friend int pick(Host) { return sizeof(T); }
};
template <class T> int pick(Host);
