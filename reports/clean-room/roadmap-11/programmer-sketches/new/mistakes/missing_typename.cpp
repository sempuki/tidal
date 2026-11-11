// A dependent type name used without typename.
template <class C> int first(const C& c) { C::value_type v = c.front(); return v; }
struct V { using value_type = int; int front() const { return 3; } };
int main() { return first(V{}) - 3; }
