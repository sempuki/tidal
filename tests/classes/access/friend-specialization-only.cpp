// tidal-rules: ACC-20
// tidal-expect: run
// Lock names one specialization of Key as a friend. Key<int> reads the
// private member.
template <class T> struct Key;
class Lock {
  int pins = 5;
  friend struct Key<int>;
};
template <class T> struct Key {
  int count(const Lock& l) const { return l.pins; }
};
int main() { return Key<int>{}.count(Lock{}) == 5 ? 0 : 1; }
