// tidal-rules: DEP-1 DEP-28
// tidal-expect: run
template <class T, int N> struct Shelf {
  using row = T[N];
  using visit = T* (*)(T&);
  typename T::size_type limit() { return T::limit; }
  T* first(row& items) { return items[0].self(); }
};
struct Item {
  using size_type = int;
  static constexpr int limit = 4;
  Item* self() { return this; }
};
int main() {
  Shelf<Item, 2> shelf;
  Item items[2];
  return shelf.limit() == 4 && shelf.first(items) == &items[0] ? 0 : 1;
}
