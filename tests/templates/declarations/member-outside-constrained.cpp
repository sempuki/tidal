// tidal-rules: TPL-23 CNS-26
// tidal-expect: run
// A member of a constrained class template defined after the class body
// repeats the whole template head: the type constraint and the clause.
template <class T> concept Small = sizeof(T) <= 8;
template <Small T> requires (sizeof(T) > 1)
struct Cell {
  T value;
  T get() const;
  static int made;
};
template <Small T> requires (sizeof(T) > 1)
T Cell<T>::get() const { return value; }
template <Small T> requires (sizeof(T) > 1)
int Cell<T>::made = 3;
int main() {
  Cell<int> c{4};
  return c.get() + Cell<int>::made == 7 ? 0 : 1;
}
