package loader

import (
	"context"
	"net/http"
	"time"
	"github.com/go-pg/pg/v9"
	"github.com/gdowding/crewd/graph"
	"github.com/gdowding/crewd/graph/model"
)

func Schedule(db *pg.DB, w http.ResponseWriter, r *http.Request, next http.Handler) {
	loader := graph.NewScheduleLoader(graph.ScheduleLoaderConfg{
		MaxBatch: 100,
		Wait: 1 * time.Millisecond,
		Fetch: func(keys []string) ([]*model.Schedule, []error) {
			var dbSchedules []*model.Schedule
			err := db.Model(&dbSchedules).WhereIn("id IN (?)", keys).Select()
			if err != nil {
				return []*model.Schedule{}, []error{err}
			}
			scheduleKeys := make(map[string]*model.Schedule)
			schedules := make([]*model.Schedule, len(keys))
			for _, schedule := range dbSchedules {
				scheduleKeys[schedule.ID] = schedule
			}
			for i, k := range keys {
				if schedule, ok := scheduleKeys[k]; ok {
					schedules[i] = schedule
				}
			}
			return schedules, []error{err}
		},
	})
	ctx := context.WithValue(r.Context(), ScheduleLoader, loader)
	next.ServeHTTP(w, r.WithContext(ctx))
}
