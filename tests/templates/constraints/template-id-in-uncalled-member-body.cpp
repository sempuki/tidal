// tidal-rules: CNS-14
// tidal-expect: run
// tidal-note: Propose X22 (reports/clean-room/X22/report.md), finding X22-1. User<char> is instantiated, and the body of size, which names Box<T>, is not. The template-id is checked when the definition that holds it is instantiated, so the program is valid.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box { T v; };
template <class T> struct User {
  int size() { return sizeof(Box<T>*); }
  int n = 0;
};
User<char> u;
int main() { return u.n; }
