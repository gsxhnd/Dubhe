package main

import (
	"fmt"
	"log"

	mqtt "github.com/eclipse/paho.mqtt.golang"
)

func main() {
	// MQTT broker address and client ID
	brokerAddress := "tcp://127.0.0.1:1883"
	clientID := "my-client"

	// Create an MQTT client options
	opts := mqtt.NewClientOptions()
	opts.AddBroker(brokerAddress)
	opts.SetClientID(clientID)
	opts.SetProtocolVersion(4)

	// Set the callback function for receiving messages
	opts.SetDefaultPublishHandler(func(client mqtt.Client, msg mqtt.Message) {
		fmt.Printf("Received message: %s\n", msg.Payload())
	})
	opts.SetOnConnectHandler(func(client mqtt.Client) {
		// fmt.Println("client id: ",client.)
		// client.
	})

	// Create an mqtt client instance
	client := mqtt.NewClient(opts)
	if token := client.Connect(); token.Wait() && token.Error() != nil {
		log.Fatal(token.Error())
	}
}
