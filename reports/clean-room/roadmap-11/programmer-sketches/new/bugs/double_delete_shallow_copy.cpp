// An owner with no copy constructor of its own, copied, so both copies delete.
struct Buf { int* p; Buf() : p(new int(1)) {} ~Buf() { delete p; } };
int main() { Buf a; { Buf b = a; } return *a.p == 1 ? 0 : 1; }
