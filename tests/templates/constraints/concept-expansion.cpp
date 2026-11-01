// tidal-rules: CNS-2 CNS-4 CNS-23
// tidal-expect: accept
// tidal-note: A2 expands to the atomic constraint of A. PtrA expands to the atomic constraint of A with T* mapped to T, which is the constraint A<T*> gives.
template <class T> concept A = sizeof(T) > 1;
template <class T> concept A2 = A<T>;
template <class T> concept PtrA = A<T*>;
template <class T> struct Box {
  static constexpr int f() requires A<T> { return 1; }
  static constexpr int f() requires A2<T> && (sizeof(T) < 100) { return 2; }
  static constexpr int g() requires A<T*> { return 1; }
  static constexpr int g() requires PtrA<T> && true { return 2; }
};
static_assert(Box<int>::f() == 2 && Box<char>::g() == 2);
