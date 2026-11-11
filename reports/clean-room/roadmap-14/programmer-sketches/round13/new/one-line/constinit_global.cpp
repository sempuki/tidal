// constinit on a global that the program changes later.
constexpr int start() { return 41; }
constinit int counter = start();
int main() { ++counter; return counter - 42; }
