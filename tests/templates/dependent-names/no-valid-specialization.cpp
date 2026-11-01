// tidal-rules: DEP-32
// tidal-expect: accept
template <class T> void never() {
  int items[sizeof(T) - sizeof(T) - 1];
  (void)sizeof(void(T));
}
