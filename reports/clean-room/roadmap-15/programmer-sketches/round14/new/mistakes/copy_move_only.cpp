struct Handle { int fd; Handle(int f) : fd(f) {} Handle(Handle&&) = default; };
int main() { Handle a(3); Handle b = a; return b.fd - 3; }
