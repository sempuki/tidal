// I5-b. A reference to a private base. GCC and Clang reject (the base is not accessible). The front end runs and returns 1. A pointer to the same base is rejected with CONV-17.
struct Base {};
struct D : private Base {};
int up(Base&) { return 1; }
int main() { D d; return up(d); }
