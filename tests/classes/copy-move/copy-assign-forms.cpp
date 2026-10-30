// tidal-rules: CPY-3
// tidal-expect: accept
struct A { A& operator=(const A&); };
struct B { B& operator=(B&); };
struct C { C& operator=(C); };
struct D { void operator=(const D&); };
struct E { E& operator=(E&&); };
