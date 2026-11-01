// tidal-rules: DCMP-13
// tidal-expect: accept
// tidal-note: No != matches its ==, so each reversed == is a candidate. The != differs in a parameter type, in its kind of function, in its scope, or is a template.
struct Wide { bool operator==(int) const; bool operator!=(long) const; };
struct Bound { bool operator==(int) const; bool operator!=(const int&) const; };
struct Outside { bool operator==(int) const; };
bool operator!=(const Outside&, int);
struct Explicit { bool operator==(int) const; bool operator!=(this const Explicit&, int); };
struct Generic { bool operator==(int) const; template <class T> bool operator!=(T) const; };
struct Copy {};
bool operator==(const Copy&, int);
bool operator!=(Copy, int);
namespace left { struct Far {}; bool operator==(const Far&, int); }
namespace right { bool operator!=(const left::Far&, int); }
using right::operator!=;
bool check(Wide w, Bound b, Outside o, Explicit e, Generic g, Copy c, left::Far f) {
  return (1 == w) && (1 == b) && (1 == o) && (1 == e) && (1 == g) && (1 == c) && (1 == f);
}
