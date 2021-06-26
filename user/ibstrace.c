#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/ioctl.h>

#include <ibstrace.h>

#define IBSTRACE_CHARDEV "/dev/ibstrace"

static struct sample *output_buf = NULL;

static uint8_t *input_buf = NULL;
static struct ibstrace_msg msg = {
	.ptr = NULL,
	.len = 0,
};

int main(int argc, char *argv[]) {
	struct stat st;
	int ibs_fd, input_fd, output_fd;
	int res, output_len, num_samples;

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
	printf("write ioctl() returned %d\n", res);
	if (res < 0) {
		close(ibs_fd);
		return -1;
	}

	// This *should* block until we're done sampling
	res = ioctl(ibs_fd, IBSTRACE_CMD_MEASURE);
	printf("measure ioctl() returned %d\n", res);

	num_samples = ioctl(ibs_fd, IBSTRACE_CMD_NUM_SAMPLE);
	if (num_samples <= 0) {
		printf("no samples collected\n");
		return -1;
	}

	if (num_samples > IBSTRACE_SAMPLE_CAPACITY) {
		printf("sample capacity %d out of bounds?\n", num_samples);
		return -1;
	}

	// Read the buffer
	output_len = num_samples * sizeof(struct sample);
	output_buf = malloc(output_len);
	res = read(ibs_fd, (void*)output_buf, output_len);
	if (res <= 0) {
		printf("read() returned %d\n", res);
		return -1;
	}
	close(ibs_fd);

	output_fd = open("/tmp/foo.bin", O_RDWR | O_CREAT );
	if (output_fd < 0) {
		printf("couldn't open output file\n");
		return -1;
	}
	write(output_fd, (void*)output_buf, output_len);
	close(output_fd);


}
