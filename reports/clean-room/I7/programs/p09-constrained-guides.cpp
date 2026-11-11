// CTAD-29: guides keep the constraints of their constructor templates.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> concept Big = sizeof(T) > 1;
template <class T> struct Holder {
  int which;
  template <class U> requires Big<U> constexpr Holder(U, T) : which(1) {}
  template <class U> requires Big<U> constexpr Holder(T, U) : which(2) {}
};
int main() {
  Holder h(10, 'a');
  static_assert(same<decltype(h), Holder<char>>);
  return h.which == 1 ? 0 : 1;
}
