using Fn = int (*)(int);
Fn pick();
int main() { return pick()(3) == 6 ? 0 : 1; }
