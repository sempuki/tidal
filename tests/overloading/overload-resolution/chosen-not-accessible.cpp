// tidal-rules: ACC-3 OVL-37
// tidal-expect: reject
// tidal-error-clang: .put. is a private member of .Item.
// tidal-error-gcc: .void Item::put\(int\). is private within this context
// tidal-error-edg: function .Item::put\(int\). .* is inaccessible
// tidal-note: Access does not affect viability. The private put(int) wins, and the public put(long) does not take its place.
class Item {
  void put(int);
public:
  void put(long);
};
void use(Item& v) { v.put(1); }
