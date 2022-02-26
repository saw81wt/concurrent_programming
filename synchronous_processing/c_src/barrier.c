#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

pthread_mutex_t barrier_mut = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t barrier_cond = PTHREAD_COND_INITIALIZER;

volatile int num = 0;
volatile int *num_p = &num;

void barrier(volatile int *cnt, int max) {
    printf("Hello. cnt: %d\n", *cnt);
    if (pthread_mutex_lock(&barrier_mut) != 0) {
        perror("pthread_mutex_lock");
        exit(-1);
    }

    //usleep(100000);
    *cnt += 1;

    if (*cnt == max) {
        printf("broad_cast_here. cnt: %d\n", *cnt);
        if (pthread_cond_broadcast(&barrier_cond) != 0) {
            perror("pthread_cond_broadcast");
            exit(-1);
        }
    } else {
        do {
            printf("Wait here. cnt: %d\n", *cnt);
            if (pthread_cond_wait(&barrier_cond, &barrier_mut) != 0) {
                perror("pthread_cond_wait");
                exit(-1);
            }
        } while (*cnt < max);
    }

    printf("bey\n");
    if (pthread_mutex_unlock(&barrier_mut) != 0) {
        perror("pthread_mutex_unlock");
        exit(-1);
    }
}


void *worker(void *arg) {
    barrier(num_p, 10);

    return NULL;
}

int main(int argc, char *argv[]) {
    pthread_t th[10];

    for (int i = 0; i < 10; ++i) {
        if (pthread_create(&th[i], NULL, worker, NULL) != 0) {
            perror("pthread_create");
            return -1;
        }
    }

    for (int i = 0; i < 10; ++i) {
        if (pthread_join(th[i], NULL) != 0) {
            perror("pthread_join");
            return -1;
        }
    }
    return 0;
}
