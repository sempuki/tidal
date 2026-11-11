// A member has no default constructor and the class does not initialize it.
struct Port { explicit Port(int n) : n(n) {} int n; };
struct Server { Port port; int load = 0; Server() {} };
int main() { Server s; return s.load; }
