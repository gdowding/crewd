package database
import (
	"gorm.io/gorm"
	"gorm.io/driver/postgres"
)

var DB *gorm.DB

func ConnectDB() error {
	dsn := "dbname=crewd"
	db, err := gorm.Open(
		postgres.Open(dsn),
		&gorm.Config{})
	if err != nil {
		return err
	}
	DB = db
	return nil
}
