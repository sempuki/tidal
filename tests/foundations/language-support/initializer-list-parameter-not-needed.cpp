// tidal-rules: LSUP-1
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: does not match
// tidal-note: No braced list initializes a std::initializer_list, so the program does not need the class. GCC checks the shape of every definition of std::initializer_list (Q-0321).
namespace std {
template <class E> class initializer_list {};
}
void take(std::initializer_list<int>);
int main() { return 0; }
