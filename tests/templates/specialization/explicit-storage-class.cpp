// tidal-rules: TSP-24
// tidal-expect: reject
// tidal-error: storage class
template <class T> int size(T) { return 0; }
template <> static int size(char) { return 1; }
