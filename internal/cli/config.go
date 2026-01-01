package cli

import (
	"fmt"
	"github.com/gdowding/crewd/ent/ent"

	_ "github.com/mattn/go-sqlite3"
)

var (
	entClient *ent.Client
)

func CreateClient() error {
	var err error
	// global entClient
	entClient, err = ent.Open("sqlite3", "file:ent?mode=memory&cache=shared&_fk=1")
	if err != nil {
		return fmt.Errorf("failed to create ent.Client: %w", err)
	}
	return nil
}

func CloseClient() {
	if entClient != nil {
		entClient.Close()
	}
}

func GetEntClient() *ent.Client {
	return entClient
}
