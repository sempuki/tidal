struct Sum {
    template <class T>
    static T operator()(T a, T b) { return a + b; }
};
int main() {
    auto f = &Sum::operator()<long>;
    return f(2, 3) == 5 ? 0 : 1;
}
