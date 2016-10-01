package main

import (
	"data"
	"fmt"
	"gopkg.in/pg.v4"
)

func initData() {
	db := pg.Connect(&pg.Options{
		Addr:     "localhost:5432",
		User:     "haokuan",
		Password: "hao123",
		Database: "tantan",
	})

	err := model.CreateSchema(db)
	if err != nil {
		fmt.Println(err)
	}

	var users []*model.User
	user1 := &model.User{
		Id:   "21341231231",
		Name: "Bob",
		Type: "user",
	}
	user2 := &model.User{
		Id:   "31231242322",
		Name: "Samantha",
		Type: "user",
	}
	user3 := &model.User{
		Id:   "41231242323",
		Name: "Tom",
		Type: "user",
	}

	users = append(users, user1)
	users = append(users, user2)
	users = append(users, user3)

	for _, user := range users {
		err := db.Create(user)
		if err != nil {
			fmt.Println(err)
		}
	}

	var relationships []*model.Relationship
	relationship1 := &model.Relationship{
		Id:      "222333444",
		From_id: "21341231231",
		To_id:   "31231242322",
		State:   "liked",
		Type:    "relationship",
	}
	relationship2 := &model.Relationship{
		Id:      "333222444",
		From_id: "31231242322",
		To_id:   "21341231231",
		State:   "disliked",
		Type:    "relationship",
	}
	relationship3 := &model.Relationship{
		Id:      "444333222",
		From_id: "31231242322",
		To_id:   "41231242323",
		State:   "matched",
		Type:    "relationship",
	}
	relationship4 := &model.Relationship{
		Id:      "555444333",
		From_id: "41231242323",
		To_id:   "31231242322",
		State:   "matched",
		Type:    "relationship",
	}
	relationships = append(relationships, relationship1)
	relationships = append(relationships, relationship2)
	relationships = append(relationships, relationship3)
	relationships = append(relationships, relationship4)

	for _, relationship := range relationships {
		err := db.Create(relationship)
		if err != nil {
			fmt.Println(err)
		}
	}

	user := model.User{}
	err = db.Model(&user).Where("id = ?", "21341231231").Select()

	if err != nil {
		fmt.Println(err)
	}

	relationship := model.Relationship{}
	err = db.Model(&relationship).Where("id = ?", "222333444").Select()

	if err != nil {
		fmt.Println(err)
	}

	var allusers []model.User
	err = db.Model(&allusers).Select()
	if err != nil {
		fmt.Println(err)
	}

	fmt.Println(user)
	fmt.Println(relationship)
	fmt.Println(allusers)
}

func main() {
	initData()
}
