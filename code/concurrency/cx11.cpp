#include <iostream>
#include <thread>

void call_from_thread() {
    std::cout << "Hello, World" << std::endl;
}

int main() {
    //Launch a thread
    std::thread t[5];

    for(int i = 0; i < 5; i++){
      t[i] = std::thread(call_from_thread);
    }

    for(int i = 0; i < 5; i++){
      t[i].join();
    }

    return 0;
}

