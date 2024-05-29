package main

import (
  "io/ioutil"
  "io"
  "os"
  "log"
  "flag"
  "bufio"
  "errors"
)

func main() {
  flag.Parse()
  args := flag.Args()
  if len(args) < 1 {
    reader := bufio.NewReader(os.Stdin)
    for {
      text, err := reader.ReadString('\n')
      if errors.Is(err, io.EOF) {
          os.Exit(0)
      } else if err != nil {
        log.Fatal(err)
      }
      print(text)
    }
  }

  contents, err := ioutil.ReadFile(args[0])
  if err != nil {
    log.Fatal(err)
  }
  print(string(contents))
}
