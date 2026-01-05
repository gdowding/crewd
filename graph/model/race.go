package model

type Race struct {
	ID string `json:"id"`
	SeriesID string `json:"tex"`
	Series *Series `json:"series"`
	Name string `json:"name"`
}
