// tidal-rules: DEP-36
// tidal-expect: accept
// tidal-note: The member N of the independent base hides the constant template parameter N, in the class and in a member defined outside it.
struct Base { static constexpr int N = 5; };
template <int N> struct Inside : Base { static constexpr int get() { return N; } };
template <int M> struct Outside : Base { static int get(); };
template <int N> int Outside<N>::get() { return N; }
static_assert(Inside<1>::get() == 5);
int y = Outside<1>::get();
