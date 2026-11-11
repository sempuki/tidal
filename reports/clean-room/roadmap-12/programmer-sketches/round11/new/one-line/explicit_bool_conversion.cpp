// An explicit operator bool used in if, &&, and !.
struct Handle { int fd; explicit operator bool() const { return fd >= 0; } };
int main() { Handle a{3}, b{-1}; if (a && !b) return 0; return 1; }
