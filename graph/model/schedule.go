package model

type Schedule struct {
	ID   string `json:"id"`
	Name string `json:"name"`
	SeriesIDs []string `json:"text"`
	Series []*Series `json:"series"`
}
