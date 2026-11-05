struct Sum {
    template <class T>
    static T add(T a, T b) { return a + b; }
};
int main() {
    auto f = &Sum::add<long>;
    return f(2, 3) == 5 ? 0 : 1;
}
