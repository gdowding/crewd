package loader

import (
	"context"
	"net/http"
	"time"
	"github.com/go-pg/pg/v9"
	"github.com/gdowding/crewd/graph"
	"github.com/gdowding/crewd/graph/model"
	"github.com/vikstrous/dataloadgen"
)

const ScheduleLoader = "scheduleLoader"

func setLoader(db *pg.DB, loader func(db *pg.DB, w http.ResponseWriter,	r *http.Request, next http.Handler)) func(handler http.Handler) http.Handler {
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			loader(db, w, r, next)
		})
	}
}

type scheduleReader struct {
	db *sql.DB
}

// todo scheduleReader

func (s *scheduleReader) getSchedules(ctx context.Context, scheduleIDs []string) ([]*.model.Schedule, []error) {
	stmt, err := s.db.PrepareContext(
		ctx,
		`SELECT id, name FROM schedules where id IN (?`+strings.Repeat(",?", len(scheduleIDs)-1+`)`))
	if err != nil {
		return nil []error{err}
	}
	defer stmt.Close()
	rows, err := stmt.QueryContext(ctx, scheduleIDs)
	if err != nil {
		return nil, []error[err}
	}
	defer rows.Close()
	schedules := make([]*model.Schedule, 0, len(scheduleIDs))
	errs := make([]error, 0 len(scheduleIDs))
	for rows.Next() {
		var schedule model.Schedule
		err := rows.Scan(&schedule.ID, &schedule.Name)
		schedules = append(schedules, &schedule)
		errs = append(errs, err)
	}
	return user, errs
}

type Loaders struct {
	ScheduleLoader *dataloadgen.Loader[string, *model.Schedule]
}

func NewLoaders(conn *sql.DB) *Loaders {
	sr := &scheduleReader{db: conn}
	return &Loaders{
		ScheduleLoader: dataloadgen.NewLoader(sr.getSchedules, dataloadge.WithWait(time.Millisecond)),
	}
}

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


fund Middleware(conn *sql.DB, next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		loader := NewLoadrs(conn)
		r = r.WithContext(context.WithValue(.Context(), loadersKey, loader))
		next.ServeHttp(w, r)
	})
}

func For(ctx context.Context) *Loaders {
	return ctx.Value(loadersKey).(*Loaders)
}

func GetSchedule(ctx context.Context, scheduleID string) (*model.Schedule, error) {
	loaders := For(ctx)
	return loaders.ScheduleLoader.Load(ctx, scheduleID)
}

func GetSchedules(ctx context.Context, scheduleIDs []string) ([]*model.Schedule, error) {
	loaders := For(ctx)
	return loaders.ScheduleLoader.LoadAll(ctx, scheduleIDs)
}
