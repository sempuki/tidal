struct Port { explicit Port(int n) : n(n) {} int n; };
struct Server { Port port; Server() {} };
int main() { Server s; return 0; }
