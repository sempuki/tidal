// tidal-rules: VIRT-13
// tidal-expect: accept
// tidal-flags: -Wall -Werror
// tidal-note: A final class has no derived class, so the deleted object has exactly that type and no warning is due.
struct Leaf final { virtual int size() const { return 1; } };
void drop(Leaf* p) { delete p; }
