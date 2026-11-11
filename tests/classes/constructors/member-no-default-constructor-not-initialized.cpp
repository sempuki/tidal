// tidal-rules: CTOR-25 INIT-9
// tidal-expect: reject
// tidal-error-clang: must explicitly initialize the member
// tidal-error-gcc: no matching function for call to .Socket::Socket\(\)
// tidal-error-edg: no default constructor exists for class
// tidal-note: Roadmap round 13, programmer reduction member_without_default_constructor_not_initialized.cpp (from the planted mistake no_default_constructor.cpp). The constructor of Listener does not name the member socket, so the member is default-initialized (INIT-9). Socket has no constructor that takes no arguments (CTOR-25). The definition of the constructor is not valid even with no object of Listener.
struct Socket {
  explicit Socket(int fd) : fd(fd) {}
  int fd;
};
struct Listener {
  Socket socket;
  Listener() {}
};
int main() { return 0; }
