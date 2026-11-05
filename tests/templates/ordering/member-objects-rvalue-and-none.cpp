// tidal-rules: ORD-6
// tidal-expect: accept
// tidal-note: Both templates are members. take(T*) && gets the object parameter Box&&. take(T) has no ref-qualifier and stands against an rvalue reference, so it gets Box&& too, and that pair ties. The second pair decides for T*. With Box& for take(T), the reference pair would prefer take(T), and the call would be ambiguous.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Ptr {}; struct Any {};
struct Box {
  template <class T> Ptr take(T*) &&;
  template <class T> Any take(T);
};
int* p = nullptr;
static_assert(same<decltype(Box{}.take(p)), Ptr>);
