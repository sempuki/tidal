// tidal-rules: ACC-2
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
class C;
struct Reader { int read(const C& c) const; int other(const C& c) const; };
class C {
  int secret = 9;
  friend int Reader::read(const C&) const;
};
int Reader::other(const C& c) const { return c.secret; }
