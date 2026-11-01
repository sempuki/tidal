// tidal-rules: DEP-32
// tidal-expect: accept
// tidal-note: The discarded branch forms an array of negative size for every T, so no argument list makes that branch valid. DEP-32 needs no diagnostic.
template <class T> int pick() {
  if constexpr (sizeof(T) > 0) {
    return 1;
  } else {
    int items[sizeof(T) - sizeof(T) - 1];
    return sizeof items;
  }
}
int x = pick<int>();
