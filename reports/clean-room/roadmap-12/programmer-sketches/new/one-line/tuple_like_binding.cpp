// Structured bindings through the tuple protocol of a class of my own.
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
struct Pt { int x; long y; template <decltype(sizeof 0) I> auto get() const { if constexpr (I == 0) return x; else return y; } };
template <> struct std::tuple_size<Pt> { static constexpr decltype(sizeof 0) value = 2; };
template <> struct std::tuple_element<0, Pt> { using type = int; };
template <> struct std::tuple_element<1, Pt> { using type = long; };
int main() { auto [a, b] = Pt{3, 4L}; return a + static_cast<int>(b) - 7; }
