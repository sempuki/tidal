namespace lib {
struct Item {};
inline int probe(Item, double) { return 1; }
inline int probe(Item, int) { return 2; }
}
template <class T> int info(T item) { return probe(item, 0); }
int there() { return info(lib::Item{}); }
