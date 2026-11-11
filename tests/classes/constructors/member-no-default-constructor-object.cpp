// tidal-rules: CTOR-25 INIT-9
// tidal-expect: reject
// tidal-error-clang: must explicitly initialize the member
// tidal-error-gcc: no matching function for call to .Socket::Socket\(\)
// tidal-error-edg: no default constructor exists for class
// tidal-note: Roadmap round 13, programmer probe member_without_default_constructor_used.cpp. The constructor of Listener does not name the member socket, so the member is default-initialized (INIT-9), and Socket has no constructor that takes no arguments (CTOR-25). The program also makes an object of Listener.
struct Socket {
  explicit Socket(int fd) : fd(fd) {}
  int fd;
};
struct Listener {
  Socket socket;
  Listener() {}
};
int main() {
  Listener l;
  return l.socket.fd;
}
