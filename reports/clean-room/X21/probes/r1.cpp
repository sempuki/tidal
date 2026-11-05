#include <cstdio>
int& right();
int& left() { static int v = right() + 1; return v; }
int main() { std::printf("start\n"); std::fflush(stdout); return left(); }
