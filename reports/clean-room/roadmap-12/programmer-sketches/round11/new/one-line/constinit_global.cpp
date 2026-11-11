// constinit on a global that the program then changes.
constexpr int start() { return 40; }
constinit int counter = start();
int bump() { return counter += 2; }
int main() { return bump() == 42 ? 0 : 1; }
