// tidal-rules: TPL-40
// tidal-expect: run
// tidal-note: The inner default of the class's head serves the class body. The member definition outside the class gives its own inner default.
template <class T> struct Box { T value; };
template <template <class = int> class C> struct Use {
  int inside() { C<> c; return sizeof(c); }
  int outside();
};
template <template <class = char> class C> int Use<C>::outside() { C<> c; return sizeof(c); }
int main() { Use<Box> u; return u.inside() == sizeof(int) && u.outside() == 1 ? 0 : 1; }
