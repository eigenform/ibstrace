#include <stdlib.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>
#include <stdint.h>


uint8_t buf[4] = { 0xde, 0xad, 0xbe, 0xef };

int main(int argc, char *argv[]) {
	int fd = open("/proc/ibstrace", O_RDWR);
	if (fd < 0) {
		printf("Couldn't open /proc/ibstrace (are you root?)\n");
		exit(-1);
	}

	write(fd, buf, 4);

	close(fd);
}
