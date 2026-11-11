// Two partial specializations, T[N] and const T[N]. For const char[4] the
// second is more specialized. The front end calls the choice ambiguous.
namespace std { using size_t = decltype(sizeof 0); }
template <class T> struct decay_of { using type = T; };
template <class T, std::size_t N> struct decay_of<T[N]> { using type = T*; };
template <class T, std::size_t N> struct decay_of<const T[N]> { using type = const T*; };
using P = decay_of<const char[4]>::type;
P p = "lit";
int main() { return p[0] == 'l' ? 0 : 1; }
