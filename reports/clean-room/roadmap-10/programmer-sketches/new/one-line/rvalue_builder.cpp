// A builder whose setters are &&-qualified and return the object by rvalue reference.
struct Request {
  int port = 0; bool tls = false;
  Request&& with_port(int p) && { port = p; return static_cast<Request&&>(*this); }
  Request&& with_tls() && { tls = true; return static_cast<Request&&>(*this); }
};
int main() {
  Request r = Request{}.with_port(443).with_tls();
  return r.port == 443 && r.tls ? 0 : 1;
}
