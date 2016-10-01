package model

import (
	"gopkg.in/pg.v4"
)

type User struct {
	Id   string
	Name string
	Type string
}

type Relationship struct {
	Id      string
	From_id string
	To_id   string
	State   string
	Type    string
}

func CreateSchema(db *pg.DB) error {
	queries := []string{
		`drop table if exists users;`,
		`create table users(id char(11) primary key, name varchar(10), type varchar(15));`,
		`drop table if exists relationships;`,
		`create table relationships(id char(9) primary key, from_id char(11), to_id char(11), state varchar(10), type varchar(15));`,
	}
	for _, q := range queries {
		_, err := db.Exec(q)
		if err != nil {
			return err
		}
	}
	return nil
}
