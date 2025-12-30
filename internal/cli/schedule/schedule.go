package schedule


import (
	"fmt"

	"github.com/spf13/cobra"

)

func newScheduleCmd() *cobra.Command {
	
	cmd := &cobra.Command{
		Use: "new",
		Short: "create a new schedule",
		RunE: newSchedule,
	}
	return cmd
}


func NewCommand() *cobra.Command {
	
	cmd := &cobra.Command{
		Use: "schedule",
		Short: "Commands related to 'schedule' entity.",
		RunE: runSchedule,
	}
	cmd.AddCommand(newScheduleCmd())
	return cmd

}

func runSchedule(cmd *cobra.Command, args []string) error {
	fmt.Println("schedule entity")
	return nil
}



func newSchedule(cmd *cobra.Command, args []string) error {
	fmt.Println("schedule new command")
	return nil
}
