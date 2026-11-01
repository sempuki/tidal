// tidal-rules: CPT-11
// tidal-expect: accept
template <class T> struct Box {
  friend constexpr int touch(Box) requires (sizeof(T) > 1) { return 1; }
};
static_assert(touch(Box<int>{}) == 1);
