package main

import (
  "io/ioutil";
)

func main() {
  contents,_ := ioutil.ReadFile("cat.go")
  print(string(contents))
}
