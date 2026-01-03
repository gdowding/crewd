package model

type Series struct {
	ID       string    `json:"id"`
	ScheduleID string  `json:"text"`
	Schedule *Schedule `json:"schedule"`
	Name     string    `json:"name"`
	Races    []*Race   `json:"races,omitempty"`
}
