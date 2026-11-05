int main() {
    int b = 3;
    auto copy = auto(b);
    copy *= 2;
    auto c = auto{b};
    return copy == 6 && c == 3 ? 0 : 1;
}
