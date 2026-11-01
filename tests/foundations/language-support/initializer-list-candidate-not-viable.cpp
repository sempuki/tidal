// tidal-rules: LSUP-1 ICS-16
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: does not match
// tidal-note: The elements do not convert to int*, so the first take has no sequence (ICS-16) and no list initializes a std::initializer_list. GCC checks the shape of every definition of std::initializer_list (Q-0321).
namespace std {
template <class E> class initializer_list {};
}
struct Point { int x, y; };
void take(std::initializer_list<int*>);
void take(Point) {}
int main() { take({1, 2}); return 0; }
