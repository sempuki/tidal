struct Sum {
    template <class T>
    static T operator()(T a, T b) { return a + b; }
};
int main() { return Sum{}(2L, 3L) == 5 ? 0 : 1; }
