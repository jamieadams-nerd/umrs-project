// SPDX-License-Identifier: MIT
//
// Minimal libselinux color proof test
//
// Build:
//   gcc -Wall -O2 color_test.c -o color_test -lselinux
//
// Run:
//   ./color_test TESTFILE
//

#include <selinux/selinux.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[]) {
  if (argc != 2) {
    fprintf(stderr, "Usage: %s <file>\n", argv[0]);
    return 1;
  }

  const char *path = argv[1];

  static const char *labels[4] = {"USER", "ROLE", "TYPE", "RANGE"};
  char *ctx = NULL;
  char *color = NULL;

  // ------------------------------------------------------------
  // 1. Get file security context
  // ------------------------------------------------------------
  // if (getfilecon(path, &ctx) < 0) {
  // perror("getfilecon");
  // return 1;
  //}


  if (getfilecon_raw(path, &ctx) < 0) {
    perror("getfilecon");
    return 1;
  }

  printf("Path        : %s\n", path);
  printf("Raw Context : %s\n", ctx);

  if (selinux_raw_context_to_color(ctx, &color) == 0 && color) {
    printf("Color String: %s\n", color);

    char *save = NULL;
    //char *tok = strtok_r(color, " ", &save);
    //int i = 0;
    //while (tok) {
      //printf("Color %d: %s\n", i++, tok);
      //tok = strtok_r(NULL, " ", &save);
    //}

    /* Show four pairs */
    save = NULL;
    for (int i = 0; i < 4; i++) {
      char *fg = strtok_r(i == 0 ? color : NULL, " ", &save);
      char *bg = strtok_r(NULL, " ", &save);
      if (!fg || !bg)
        break;
      printf("%-5s FG %s  BG %s\n", labels[i], fg, bg);
    }

    free(color);

  } else {
    printf("Color String: <none>\n");
  }
  freecon(ctx);
  return 0;
}
