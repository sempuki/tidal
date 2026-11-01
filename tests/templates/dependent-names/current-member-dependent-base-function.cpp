// tidal-rules: DEP-18
// tidal-expect: accept
// tidal-note: Box::make finds nothing in Box at the definition. Box has the dependent base Base<T>, so the name is looked up at the instantiation and finds Base<int>::make.
template <class T> struct Base { static constexpr int make() { return 5; } };
template <class T> struct Box : Base<T> {
  static constexpr int get() { return Box::make(); }
};
static_assert(Box<int>::get() == 5);
