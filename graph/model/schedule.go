package model

type Schedule struct {
	ID   string `json:"id"`
	Name string `json:"name"`
	Series []*Series `json:"series,omitempty"`
}
