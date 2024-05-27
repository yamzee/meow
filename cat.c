#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(int argc, char **argv){
  int opt;

  while ((opt = getopt(argc, argv, "h")) != -1){
    switch(opt){
      case 'h':
        printf("hi lol\n");
        exit(1);
    }
  }

  FILE *fp;
  const int buffSize = 4096;
  char buff[buffSize];
  int currFile = (argc > 1 ? 1 : 0);
  
  while (currFile < argc){
    if (argc > 1){
      fp = fopen(argv[currFile], "rb");
      if (fp == NULL){
        fprintf(stderr, "%s: %s: No such file or dir.\n", argv[0], argv[currFile]);
        exit(1);
      }
    }

    while (fgets(buff, buffSize, (fp == NULL ? stdin : fp))){
      int len = strlen(buff);
      buff[len - 1] = '\0';
      fprintf(stdout, "%s\n", buff);
    }

    fclose(fp);
    currFile++;
  }

  return 0;
}
