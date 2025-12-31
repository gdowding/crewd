package schema

import (
	"entgo.io/ent"
	"entgo.io/ent/schema/field"
)

// Schedule holds the schema definition for the Schedule entity.
type Schedule struct {
	ent.Schema
}

// Fields of the Schedule.
func (Schedule) Fields() []ent.Field {
	return []ent.Field{
		field.String("name"),
	}
}

// Edges of the Schedule.
func (Schedule) Edges() []ent.Edge {
	return nil
}
