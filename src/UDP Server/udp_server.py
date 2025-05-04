import socket

def main():
    # Create a UDP socket
    udp_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

    # Bind the socket to the specified address and port
    server_address = ('0.0.0.0', 8081)  # Listen on all interfaces (replace with your desired port)
    udp_socket.bind(server_address)

    print(f"UDP server listening on {server_address}...")

    try:
        while True:
            # Receive data from the client
            data, client_address = udp_socket.recvfrom(4096)  # Buffer size is 4096 bytes
            print(f"Received message from {client_address}: {data.decode('utf-8')}")
    except KeyboardInterrupt:
        print("Server stopped.")
    finally:
        udp_socket.close()

if __name__ == "__main__":
    main()