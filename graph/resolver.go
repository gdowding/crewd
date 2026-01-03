package graph

//go:generate go tool gqlgen generate

// This file will not be regenerated automatically.
//
// It serves as dependency injection for your app, add any dependencies you require
// here.

import (
	"github.com/gdowding/crewd/graph/model"
)

type Resolver struct{
	// TODO change to map of references
	Schedules_ map[string]model.Schedule
	Series_ map[string]model.Series
	Races_ map[string]model.Race
}
