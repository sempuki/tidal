// tidal-rules: CNS-32 CNS-24
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: call to 'touch' is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-note: The friend of Box<int> and the namespace template are two templates, and neither is more constrained. EDG treats them as one template (Q-0268).
template <class T> struct Box {
  template <class U> friend constexpr int touch(Box<int>, U) requires (sizeof(T) > 1) { return 1; }
};
template <class U> constexpr int touch(Box<int>, U) requires (sizeof(int) > 1);
int chosen = touch(Box<int>{}, 1);
