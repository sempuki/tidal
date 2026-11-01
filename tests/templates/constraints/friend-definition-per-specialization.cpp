// tidal-rules: CNS-32
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-note: Box<int> and Box<double> each define touch(int). The trailing clause names T, so the two definitions are two functions. EDG reports a second definition (Q-0268).
template <class T> struct Box {
  friend constexpr int touch(int) requires (sizeof(T) > 1) { return sizeof(T); }
};
Box<int> first;
Box<double> second;
