struct Ord {
    signed char v;
    friend bool operator<(Ord a, decltype(nullptr)) { return a.v < 0; }
};
int main() {
    Ord o{-1};
    return o < 0 ? 0 : 1;
}
