struct Id {
    template <class T>
    T operator()(T a) const { return a; }
};
int main() { return Id{}(3) == 3 ? 0 : 1; }
