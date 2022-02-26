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

    pthread_mutex_lock(&mut);
    ready = true;

    if (pthread_cond_broadcast(&cond) != 0) {
        perror("pthread_cond_broadcast");
        exit(1);
    }
    
    printf("here1\n");
    usleep(1000000);

    // lockが解放されるとconsumerでのpthread_cond_waitでlockを獲得すると理解
    pthread_mutex_unlock(&mut);
    return NULL;
}

void* consumer(void *args) {
    pthread_mutex_lock(&mut);

    while (!ready) {
        if (pthread_cond_wait(&cond, &mut) != 0) {
            perror("pthread_cond_wait");
            exit(-1);
        }
    }

    pthread_mutex_unlock(&mut);
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
