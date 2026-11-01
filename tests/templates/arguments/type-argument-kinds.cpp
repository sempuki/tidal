// tidal-rules: TARG-25
// tidal-expect: accept
// tidal-note: A local class, an unnamed enumeration, an unnamed class, and an incomplete class as type arguments.
template <class T> struct Box { T* pointer = nullptr; };
struct Incomplete;
enum { first, second } unnamed_enum_value;
struct { int x; } unnamed_class_value;
int use() {
  struct Local { int v; };
  Box<Local> a;
  Box<Incomplete> b;
  Box<decltype(unnamed_enum_value)> c;
  Box<decltype(unnamed_class_value)> d;
  return a.pointer == nullptr && b.pointer == nullptr && c.pointer == nullptr && d.pointer == nullptr;
}
