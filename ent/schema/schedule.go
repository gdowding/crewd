package schema

import "entgo.io/ent"

// Schedule holds the schema definition for the Schedule entity.
type Schedule struct {
	ent.Schema
}

// Fields of the Schedule.
func (Schedule) Fields() []ent.Field {
	return nil
}

// Edges of the Schedule.
func (Schedule) Edges() []ent.Edge {
	return nil
}
