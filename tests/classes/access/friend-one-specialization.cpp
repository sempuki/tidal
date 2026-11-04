// tidal-rules: ACC-2 ACC-20
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
// Lock names one specialization of Key as a friend. Key<long> is not a
// friend, so its use of the private member is rejected.
template <class T> struct Key;
class Lock {
  int pins = 5;
  friend struct Key<int>;
};
template <class T> struct Key {
  int count(const Lock& l) const { return l.pins; }
};
int main() {
  Key<int> good;
  Key<long> bad;
  return good.count(Lock{}) + bad.count(Lock{});
}
