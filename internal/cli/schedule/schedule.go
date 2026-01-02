package schedule


import (

	"github.com/spf13/cobra"

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
	return nil
}
