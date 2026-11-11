// A member of a class that is only declared is used.
struct Impl;
struct Handle { Impl* p; int size() const { return p->size; } };
struct Impl { int size; };
int main() { Impl i{3}; Handle h{&i}; return h.size() - 3; }
