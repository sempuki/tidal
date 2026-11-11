// A copy of a class whose copy constructor is deleted.
struct Conn { Conn() = default; Conn(const Conn&) = delete; Conn(Conn&&) = default; };
int main() { Conn a; Conn b = a; (void)b; return 0; }
