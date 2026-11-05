struct Sum {
    static long operator()(long a, long b) { return a + b; }
};
int main() {
    auto f = &Sum::operator();
    return f(2, 3) == 5 ? 0 : 1;
}
