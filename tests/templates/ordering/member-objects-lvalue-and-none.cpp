// tidal-rules: ORD-6
// tidal-expect: accept
// tidal-note: Both templates are members. take(T*) & gets the object parameter Box&, and take(T), with no ref-qualifier, gets Box& too. The object pair ties, and the second pair decides for T*.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Ptr {}; struct Any {};
struct Box {
  template <class T> Ptr take(T*) &;
  template <class T> Any take(T);
};
Box box;
int* p = nullptr;
static_assert(same<decltype(box.take(p)), Ptr>);
