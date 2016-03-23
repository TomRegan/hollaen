package main;

import (
	"net"
	"fmt"
)

func quack_handler() int32 {
	sock, err := net.Listen("tcp", ":8080")
	if err != nil {
		return 1;
	}
	fmt.Println("---accepting connections on 8080")
	for {
		conn, err := sock.Accept()
		if err != nil {
			fmt.Println("---oops: shit")
			continue
		}
		buf := make([]byte, 0xff)
		n, err := conn.Read(buf)
		if err != nil {
			fmt.Println("---oops: balls")
			continue
		}
		fmt.Printf("> %s", buf)
		fmt.Printf("---RX %v bytes\n", n)
		conn.Write(buf)
		fmt.Printf("< %s", buf)
		fmt.Printf("---TX %v bytes\n", n)
		conn.Close()
	}
	return 0;
}

func main() {
	quack_handler();
}
