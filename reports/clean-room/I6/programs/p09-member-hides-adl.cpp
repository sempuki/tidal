// DEP-14: an unqualified call that finds a member at the definition keeps the member.
namespace lib { struct Item {}; int size(Item) { return 9; } }
template <class T> struct Bag {
  static int size(T) { return 4; }
  int count(T t) const { return size(t); }
};
int main() { return Bag<lib::Item>{}.count(lib::Item{}) == 4 ? 0 : 1; }
