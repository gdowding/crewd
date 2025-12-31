/*
Copyright © 2025 NAME HERE <EMAIL ADDRESS>

*/
package main

import (
	"context"
	"log"
	
	"github.com/gdowding/crewd/ent/ent"
	
	_ "github.com/mattn/go-sqlite3"
	"github.com/gdowding/crewd/cmd"
)


func main() {
	client, err := ent.Open("sqlite3", "file:ent?mode=memory&cache=shared&_fk=1")
	if err != nil {
		log.Fatalf("failed opening connection to sqlite: %v", err)
	}
	defer client.Close()
	// Run the auto migration tool.
	if err := client.Schema.Create(context.Background()); err != nil {
		log.Fatalf("failed creating schema resources: %v", err)
	}
	log.Print("created schema resources")
	cmd.Execute()	
}
