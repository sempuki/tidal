// tidal-rules: DCMP-13
// tidal-expect: accept
// tidal-note: The != is a friend declared only in the class. A search of the namespace does not find it, so the != does not match and the reversed == is a candidate.
struct Hidden {
  friend bool operator==(const Hidden&, int);
  friend bool operator!=(const Hidden&, int);
};
bool check(Hidden h) { return 1 == h; }
