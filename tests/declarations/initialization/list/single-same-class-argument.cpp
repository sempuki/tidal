// tidal-rules: LIST-10
// tidal-expect: run
// tidal-note: A braced list with one object of the aggregate's class initializes a function parameter. The parameter is a copy of the object, and no constructor of the member runs on the whole object.
struct A;
struct Holder { int origin; Holder(int o) : origin(o) {} Holder(const A&); };
struct A { Holder h; };
Holder::Holder(const A&) : origin(2) {}
int read(A a) { return a.h.origin; }
int main() {
  A a{Holder{1}};
  return read({a}) == 1 ? 0 : 1;
}
