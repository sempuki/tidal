// DEP-9, NAM-27: argument-dependent lookup through a template argument's namespace.
namespace lib { struct Item {}; }
template <class T> struct Box { T held; };
template <class T> int use(T box) { return probe(box); }
namespace lib { int probe(Box<Item>) { return 7; } }
int main() { return use(Box<lib::Item>{}) == 7 ? 0 : 1; }
