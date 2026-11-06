// I5-c. A reference to an ambiguous base. GCC and Clang reject. The front end runs and returns 1.
struct Base {};
struct L : Base {};
struct R : Base {};
struct Both : L, R {};
int up(Base&) { return 1; }
int main() { Both b; return up(b); }
