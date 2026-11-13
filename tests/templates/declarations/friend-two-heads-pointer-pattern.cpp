// tidal-rules: TPL-4
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: declared as friend before type
// tidal-error-edg: is incompatible with
// tidal-note: RV12-4. Q-0458. A friend declaration has two template heads, and its qualifier Shelf<T*> gives the argument T*, a pattern of the first head's parameter (TPL-4). GCC and EDG reject the declaration. Clang accepts it and lets the member peek of Shelf<int*> read the private member of Vault.
template <class T> struct Shelf {
  template <class U> constexpr int peek(U);
};
class Vault {
  int code = 7;
  template <class T> template <class U> friend constexpr int Shelf<T*>::peek(U);
};
template <class T> template <class U> constexpr int Shelf<T>::peek(U) { return Vault{}.code; }
static_assert(Shelf<int*>{}.peek('a') == 7);
