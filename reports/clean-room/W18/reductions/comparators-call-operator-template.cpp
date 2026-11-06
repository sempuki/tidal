struct G { template <class T> int operator()(T a) const { return a; } };
int main() { G g; return g(0); }
