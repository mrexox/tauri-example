package main

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"net"
	"os"
	"strings"
)

const maxConcurrentHandlers = 1

type Message struct {
	Event string `json:"event"`
	Data  json.RawMessage
}

func main() {
	slog.SetDefault(slog.New(slog.NewTextHandler(os.Stdout, nil)))

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	var lc net.ListenConfig
	listener, err := lc.Listen(ctx, "tcp", "")
	if err != nil {
		slog.Error("failed to start the listener", "err", err)
		return
	}

	defer listener.Close()
	addr := listener.Addr().String()

	fmt.Println(addr[strings.LastIndex(addr, ":")+1:])

	slog.Info("start server at " + addr)
	done := make(chan struct{})
	go func() {
		conn, err := listener.Accept()
		if err != nil {
			slog.Error("failed to accept the connection", "err", err)
			done <- struct{}{}
			return
		}
		slog.Info("new connection", "remote_addr", conn.RemoteAddr())

		handleConnection(ctx, conn)

		slog.Info("connection closed")
		done <- struct{}{}
	}()

	select {
	case <-ctx.Done():
	case <-done:
	}

	return
}

func handleConnection(ctx context.Context, conn net.Conn) {
	defer conn.Close()
	eof := make(chan struct{})

	go func() {
		sem := make(chan struct{}, maxConcurrentHandlers)
		scan := bufio.NewScanner(conn)

		// NOTE: Scan() uses "\n" as a divider. If there are problems with reading messages
		// consider changing the message divider on reading: provide a different split func.
		//
		//   scan.Split(splitFn)
		//
		for scan.Scan() {
			bytes := scan.Bytes()

			var message Message
			if err := json.Unmarshal(bytes, &message); err != nil {
				slog.Error("failed to unmarshal the message from desktop", "message", bytes)
				continue
			}

			sem <- struct{}{}
			go func() {
				// handle(ctx, message)
				slog.Info("new message", "event", message.Event, "data", string(message.Data))
				<-sem
			}()
		}
		eof <- struct{}{}
	}()

	select {
	case <-ctx.Done():
	case <-eof:
	}
}
