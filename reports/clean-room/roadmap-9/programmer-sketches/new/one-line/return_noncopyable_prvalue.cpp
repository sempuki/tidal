struct Pinned { int v; Pinned(int x) : v(x) {} Pinned(const Pinned&) = delete; Pinned(Pinned&&) = delete; };
Pinned make(int x) { return Pinned(x * 2); }
int main() { Pinned p = make(21); return p.v == 42 ? 0 : 1; }
