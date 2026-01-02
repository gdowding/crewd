package graph

// This file will not be regenerated automatically.
//
// It serves as dependency injection for your app, add any dependencies you require
// here.

import (
	"github.com/gdowding/crewd/graph/model"
)

type Resolver struct{
	schedules []*model.Schedule
	series []*model.Series
	races []*model.Race
}
