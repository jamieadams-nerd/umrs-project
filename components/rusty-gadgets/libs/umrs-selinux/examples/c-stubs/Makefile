# SPDX-License-Identifier: MIT
# Author: Jamie Adams (a.k.a, Imodium Operator)
#
# Makefile build c-stubs for testing

CC      := gcc
CFLAGS  := -Wall -Wextra -O2
LDFLAGS := -lselinux

SRC  := $(wildcard *.c)
BIN  := $(SRC:.c=)

# Default target
all: $(BIN)

# Pattern rule — build each binary from its .c
%: %.c
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS)

# Clean build artifacts
clean:
	rm -f $(BIN)

.PHONY: all clean
