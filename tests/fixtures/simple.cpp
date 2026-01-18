// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

// Simple C++ test fixture for parser tests

#include <iostream>
#include <string>

class Greeter {
public:
    Greeter(const std::string& name) : name_(name) {}
    
    void greet() const {
        std::cout << "Hello, " << name_ << "!" << std::endl;
    }
    
private:
    std::string name_;
};

template<typename T>
T square(T x) {
    return x * x;
}

int main() {
    Greeter g("Axiom");
    g.greet();
    
    int result = square(5);
    std::cout << "5^2 = " << result << std::endl;
    
    return 0;
}
