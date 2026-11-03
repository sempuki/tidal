auto divide(int a, int b) { struct Out { int q, r; }; return Out{a / b, a % b}; }
int main(int argc, char**) { auto [q, r] = divide(7 + argc, 3); return q - 2 + r - 2; }
