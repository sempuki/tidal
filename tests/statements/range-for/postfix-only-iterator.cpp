// tidal-rules: RFOR-2
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
struct It {
  int v;
  int operator*() const { return v; }
  It operator++(int) { It old = *this; ++v; return old; }
  bool operator!=(const It& o) const { return v != o.v; }
};
struct R { It begin() { return {0}; } It end() { return {3}; } };
void f() { for (int x : R{}) (void)x; }
