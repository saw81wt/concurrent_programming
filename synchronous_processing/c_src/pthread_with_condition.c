#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>
#include <unistd.h>

pthread_mutex_t mut = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t cond = PTHREAD_COND_INITIALIZER;

volatile bool ready = false;
char buf[256];

void* producer(void *args) {
    printf("producer: ");
    fgets(buf, sizeof(buf), stdin);

    // printf("here3\n");
    pthread_mutex_lock(&mut);

    // fgetsをここに書いてしまってproducerが先にlockを獲得してしまった場合
    // consumerはpthread_mutex_lockで待たされる
    // lockを獲得できる頃にはready = trueになっているので、here2を通ることなく消費を行う。
    // fgets(buf, sizeof(buf), stdin);
    ready = true;

    if (pthread_cond_broadcast(&cond) != 0) {
        perror("pthread_cond_broadcast");
        exit(1);
    }
    
    // printf("here4\n");
    usleep(1000000);

    // lockが解放されるとconsumerでのpthread_cond_waitでlockを獲得すると理解
    pthread_mutex_unlock(&mut);
    return NULL;
}

void* consumer(void *args) {
    // usleep(1000000);
    // printf("here1\n");
    pthread_mutex_lock(&mut);

    while (!ready) {
        // printf("here2\n");
        if (pthread_cond_wait(&cond, &mut) != 0) {
            perror("pthread_cond_wait");
            exit(-1);
        }
    }

    pthread_mutex_unlock(&mut);
    // printf("here5\n");
    printf("consumer: %s\n", buf);
    return NULL;
}

int main(int argc, char *argv[]) {
    pthread_t pr, cn;
    pthread_create(&pr, NULL, producer, NULL);
    pthread_create(&cn, NULL, consumer, NULL);

    pthread_join(pr, NULL);
    pthread_join(cn, NULL);

    pthread_mutex_destroy(&mut);

    if(pthread_cond_destroy(&cond) != 0) {
        perror("pthread_cond_destroy");
        return -1;
    }

    return 0;
}
