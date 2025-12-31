/*
Copyright © 2025 George Dowding <pyrios@gmail.com>

*/
package main

import (
	"context"
	"fmt"
	"log"

	"github.com/gdowding/crewd/ent/ent"
	"github.com/gdowding/crewd/ent/ent/schedule"

	_ "github.com/mattn/go-sqlite3"
	//"github.com/gdowding/crewd/cmd"
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
	s, err := CreateSchedule(context.Background(), client)
	if err != nil {
		log.Fatalf("failed creating schedule: %v", err)
	}
	log.Println("new schedule: ", s)
	qs, err := QuerySchedule(context.Background(), client)
	if err != nil {
		log.Fatalf("failed quering schedule: %v", err)
	}
	log.Println("retreived schedule: ", qs)
	//cmd.Execute()
}

func CreateSchedule(ctx context.Context, client *ent.Client) (*ent.Schedule, error) {
	s, err := client.Schedule.
		Create().
		SetName("irie").
		Save(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed creating schedule: %w", err)
	}
	return s, nil
}

func QuerySchedule(ctx context.Context, client *ent.Client) (*ent.Schedule, error) {
    u, err := client.Schedule.
        Query().
        Where(schedule.Name("irie")).
        // `Only` fails if no schedule found,
        // or more than 1 schedule returned.
        Only(ctx)
    if err != nil {
        return nil, fmt.Errorf("failed querying schedule: %w", err)
    }
    log.Println("schedule returned: ", u)
    return u, nil
}
