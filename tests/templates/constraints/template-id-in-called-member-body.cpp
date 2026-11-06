// tidal-rules: CNS-14 CNS-13
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied
// tidal-error-gcc: constraints not satisfied
// tidal-note: Propose X22 (reports/clean-room/X22/report.md), finding X22-1. The call of size instantiates its body, and Box<char> fails the constraint of Box.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box { T v; };
template <class T> struct User {
  int size() { return sizeof(Box<T>*); }
};
User<char> u;
int main() { return u.size(); }
