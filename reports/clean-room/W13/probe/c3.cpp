extern "C" int t(int v);
namespace d { extern "C" int t(int v); }
int t(int v) { return 3 * v; }
int main() { return d::t(0); }
