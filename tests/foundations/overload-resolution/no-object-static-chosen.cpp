// tidal-rules: OVL-3
// tidal-expect: accept
// tidal-note: The same set as no-object-nonstatic-chosen.cpp. The static winner needs no object.
struct Item {
  void reset();
  static void reset(int);
  static void clear() { reset(0); }
};
