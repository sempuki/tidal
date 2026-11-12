// tidal-rules: TPL-4
// tidal-expect: accept
// tidal-note: RV11-R-1. A friend declaration in a class has two template heads. The first head belongs to the class template Shelf, whose qualifier Shelf<T> uses its parameter. The friend is a member template of each specialization of Shelf, a function in one class and a nested class in the other. Every compiler accepts both.
template <class T> struct Shelf {
  template <class U> constexpr int peek(U);
  template <class U> struct Lens;
};
class Vault {
  int code = 7;
  template <class T> template <class U> friend constexpr int Shelf<T>::peek(U);
  template <class T> template <class U> friend struct Shelf<T>::Lens;
};
template <class T> template <class U> constexpr int Shelf<T>::peek(U) { return Vault{}.code; }
template <class T> template <class U> struct Shelf<T>::Lens {
  constexpr int look() const { return Vault{}.code + 1; }
};
static_assert(Shelf<int>{}.peek('a') == 7);
static_assert(Shelf<long>::Lens<char>{}.look() == 8);
