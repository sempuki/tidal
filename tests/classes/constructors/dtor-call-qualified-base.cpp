// tidal-rules: DTOR-8
// tidal-expect: accept
// tidal-note: Gap review RV7-A-1. In an explicit destructor call, a qualified name of a base class after ~ names the destructor of that base. Every compiler accepts the call on a Derived object and through a Derived pointer (writer probe roadmap-11/writer-probes/rv7-points.json). The functions are never called.
struct Base { ~Base() {} };
struct Derived : Base { int count = 0; };
void end_base(Derived& item) { item.Base::~Base(); }
void end_base_through(Derived* item) { item->Base::~Base(); }
