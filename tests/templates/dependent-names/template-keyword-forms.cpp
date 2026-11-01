// tidal-rules: DEP-24 DEP-25
// tidal-expect: run
template <template <class> class TT> struct Holder { static constexpr int value = 1; };
struct Has {
  template <class U> static int make() { return sizeof(U); }
  template <class U> int get() { return 2; }
  template <class U> struct box { U v; };
  template <class U> using pointer = U*;
};
template <class T> int run(T object, T* address) {
  typename T::template box<int> b{3};
  typename T::template pointer<int> p = &b.v;
  return T::template make<char>() + object.template get<int>()
       + address->template get<long>() + *p
       + Holder<T::template box>::value;
}
int plain() {
  Has h;
  return Has::template make<char>() + h.template get<int>();
}
int main() {
  Has h;
  return run(h, &h) == 1 + 2 + 2 + 3 + 1 && plain() == 3 ? 0 : 1;
}
