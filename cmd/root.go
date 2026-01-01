/*
Copyright © 2025 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (
	"os"
	//"log"
	"fmt"

	"github.com/spf13/cobra"
	"github.com/gdowding/crewd/internal/cli"
	"github.com/gdowding/crewd/internal/cli/schedule"
)

var (
	// flags
	engine string
	url string
)


// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "crewd",
	Short: "Crew managment package",
	Long: `crewd is a crew management package.`,
	// Uncomment the following line if your bare application
	// has an action associated with it:
	// Run: func(cmd *cobra.Command, args []string) { },
	PersistentPreRunE: persistentPreRunE,
	PersistentPostRun: persistentPostRun,
	// TODO: shutdown client PostRun?
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	// Here you will define your flags and configuration settings.
	// Cobra supports persistent flags, which, if defined here,
	// will be global for your application.

	// rootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "config file (default is $HOME/.crewd.yaml)")
	rootCmd.PersistentFlags().StringVar(&engine, "engine", "", "engine for backend (default is sqlite3)")
	rootCmd.PersistentFlags().StringVar(&url, "url", "",
		"Configuration URL for engine (default is file:ent?mode=memory&cache=shared&_fk=1)")

	// Cobra also supports local flags, which will only run
	// when this action is called directly.
	rootCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
	rootCmd.AddCommand(schedule.NewCommand())
}

func persistentPreRunE(cmd *cobra.Command, args []string) error {
	err := cli.CreateClient()
	if err != nil {
		return fmt.Errorf("error creating client: %w", err)
	}
	return nil

}

func persistentPostRun(cmd *cobra.Command, args []string) {
	cli.CloseClient()
}
