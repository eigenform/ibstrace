#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/ioctl.h>

#include <ibstrace.h>

#define IBSTRACE_CHARDEV "/dev/ibstrace"

static uint8_t *input_buf = NULL;
static struct ibstrace_msg msg = {
	.ptr = NULL,
	.len = 0,
};

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

	// Submit message to driver
	msg.ptr = input_buf;
	msg.len = st.st_size;
	ibs_fd = open(IBSTRACE_CHARDEV, O_RDWR);
	if (ibs_fd < 0) {
		printf("Couldn't open %s (are you root?)\n", IBSTRACE_CHARDEV);
		return -1;
	}

	res = ioctl(ibs_fd, IBSTRACE_CMD_WRITE, &msg);
	if (res < 0) {
		printf("write ioctl() returned %d\n", res);
		close(ibs_fd);
		return -1;
	}

	res = ioctl(ibs_fd, IBSTRACE_CMD_MEASURE);
	printf("measure ioctl() returned %d\n", res);
	close(ibs_fd);
}
