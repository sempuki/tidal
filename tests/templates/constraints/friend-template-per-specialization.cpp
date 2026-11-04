// tidal-rules: CNS-32
// tidal-expect: accept
// tidal-note: Box<int> and Box<double> each define the friend template touch. The constraint names T, so the two definitions are two templates and do not conflict (Q-0268 concerns friends that are not templates).
template <class T> struct Box {
  template <class U> friend constexpr int touch(Box<int>, U) requires (sizeof(T) > 1) { return sizeof(T); }
};
Box<int> first;
Box<double> second;
