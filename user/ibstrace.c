#include <stdlib.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>
#include <stdint.h>

static uint8_t *input_buf = NULL;

int main(int argc, char *argv[]) {
	struct stat st;
	int res, ibs_fd, input_fd;

	if (argc < 2) {
		printf("usage: %s <binary file>\n", argv[0]);
		return -1;
	}

	// Read the provided file
	res = stat(argv[1], &st);
	if (res < 0) {
		printf("Couldn't stat() input file %s\n", argv[1]);
		return -1;
	}
	input_buf = (uint8_t*)malloc(st.st_size);
	input_fd = open(argv[1], O_RDONLY);
	res = read(input_fd, input_buf, st.st_size);
	if (res != st.st_size) {
		printf("Couldn't read input file %s\n", argv[1]);
		return -1;
	}

	// Submit input buffer to the kernel
	ibs_fd = open("/proc/ibstrace", O_RDWR);
	if (ibs_fd < 0) {
		printf("Couldn't open /proc/ibstrace (are you root?)\n");
		return -1;
	}
	write(ibs_fd, input_buf, st.st_size);
	close(ibs_fd);

}
