package schedule


import (
	"context"
	"fmt"
	"log"

	"github.com/spf13/cobra"
	"github.com/gdowding/crewd/internal/cli"
	"github.com/gdowding/crewd/ent/ent"
	"github.com/gdowding/crewd/ent/ent/schedule"

)

func newScheduleCmd() *cobra.Command {

	cmd := &cobra.Command{
		Use: "new NAME",
		Short: "create a new schedule",
		RunE: newSchedule,
		Args: cobra.ExactArgs(1),
	}
	return cmd
}


func NewCommand() *cobra.Command {

	cmd := &cobra.Command{
		Use: "schedule",
		Short: "Commands related to 'schedule' entity.",
		//RunE: runSchedule,
	}
	cmd.AddCommand(newScheduleCmd())
	return cmd

}

// func runSchedule(_ *cobra.Command, _ []string) error {
// 	fmt.Println("schedule entity")
// 	return nil
// }



func newSchedule(cmd *cobra.Command, args []string) error {
	client := cli.GetEntClient()
	fmt.Printf("schedule new command using client: %v", *client)
	// Run the auto migration tool.
	if err := client.Schema.Create(context.Background()); err != nil {
		log.Fatalf("failed creating schema resources: %v", err)
	}
	log.Print("created schema resources")
	name := args[0]
	s, err := createSchedule(context.Background(), client, name)
	if err != nil {
		log.Fatalf("failed creating schedule: %v", err)
	}
	log.Println("new schedule: ", s)
	qs, err := querySchedule(context.Background(), client, name)
	if err != nil {
		log.Fatalf("failed quering schedule: %v", err)
	}
	log.Println("retreived schedule: ", qs)
	return nil
}

func createSchedule(ctx context.Context, client *ent.Client, name string) (*ent.Schedule, error) {
	s, err := client.Schedule.
		Create().
		SetName(name).
		Save(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed creating schedule: %w", err)
	}
	return s, nil
}

func querySchedule(ctx context.Context, client *ent.Client, name string) (*ent.Schedule, error) {
    u, err := client.Schedule.
        Query().
        Where(schedule.Name(name)).
        // `Only` fails if no schedule found,
        // or more than 1 schedule returned.
        Only(ctx)
    if err != nil {
        return nil, fmt.Errorf("failed querying schedule: %w", err)
    }
    log.Println("schedule returned: ", u)
    return u, nil
}
