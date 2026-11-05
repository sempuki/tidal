// Unit b without P15:  int slots = config * 2;
#include <cstdio>
extern int config;
int slots = config * 2;
int main() { std::printf("slots=%d\n", slots); std::fflush(stdout); return 0; }
