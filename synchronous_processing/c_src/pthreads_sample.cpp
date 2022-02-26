#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>
#include <chrono>
#include <thread>

pthread_mutex_t mut = PTHREAD_MUTEX_INITIALIZER;

void* some_func(void *arg) {
    if (pthread_mutex_lock(&mut) != 0) {
        perror("thread_mutex_lock");
        exit(1);
    }

    // クリティカルセクション
    printf("hello\n");
    std::this_thread::sleep_for(std::chrono::milliseconds(1000));
    printf("bey\n");

    if (pthread_mutex_unlock(&mut) != 0) {
        perror("pthread_mutex_unlock");
        exit(1);
    }

    return NULL;
}

int main(int argc, char *argv[]) {
    pthread_t th1, th2;

    if(pthread_create(&th1, NULL, some_func, NULL) != 0) {
        perror("pthread_create");
        return -1;
    }

    if(pthread_create(&th2, NULL, some_func, NULL) != 0) {
        perror("pthread_create");
        return -1;
    }

    if(pthread_join(th1, NULL) != 0) {
        perror("pthread_join");
        return -1;
    }

    if(pthread_join(th2, NULL) != 0) {
        perror("pthread_join");
        return -1;
    }

    if(pthread_mutex_destroy(&mut) != 0) {
        perror("pthread_mutex_destroy");
        return -1;
    }

    return 0;
}
