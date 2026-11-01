// tidal-rules: INST-31 INST-32
// tidal-expect: accept
// tidal-note: The virtual function area has a point right after the point of Shape<Item>, where measure is not declared, and a point at the end of the unit, where measure is declared. INST-32 allows any result, and every compiler accepts the program.
namespace lib { struct Item {}; }
template <class T> struct Shape { virtual int area() { return measure(T{}); } };
Shape<lib::Item> s;
namespace lib { int measure(Item) { return 4; } }
