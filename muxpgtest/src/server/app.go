package main

import (
	"data"
	"encoding/json"
	"github.com/gorilla/context"
	"github.com/gorilla/mux"
	"gopkg.in/pg.v4"
	"logger"
	"net/http"
	"time"
)

type Env struct {
	DB   *pg.DB
	Port string
	Host string
}

type Handler struct {
	*Env
	H func(e *Env, w http.ResponseWriter, r *http.Request) error
}

var handler *Handler

type DBconn struct {
	db *pg.DB
}

func DBPool() (*DBconn, error) {
	db := pg.Connect(&pg.Options{
		Addr:     "localhost:5432",
		User:     "haokuan",
		Password: "hao123",
		Database: "tantan",
	})
	return &DBconn{db: db}, nil
}
func (conn *DBconn) Close() {
	conn.db.Close()
}

func (conn *DBconn) WithData(fn http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		context.Set(r, "db", conn.db)
		fn(w, r)
	}
}

func HelloHandler(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Hello world!\n"))
}

func UsersInfoHandler(w http.ResponseWriter, r *http.Request) {
	db := context.Get(r, "db").(*pg.DB)

	var users []model.User
	err := db.Model(&users).Select()
	if err != nil {
		logger.Error(err.Error())
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(users); err != nil {
		logger.Error(err.Error())
		http.Error(w, err.Error(), http.StatusBadRequest)
	}
}

func CreateUsersHandler(w http.ResponseWriter, r *http.Request) {
	user := model.User{}
	decoder := json.NewDecoder(r.Body)
	err := decoder.Decode(&user)
	if err != nil {
		logger.Error(err.Error())
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	db := context.Get(r, "db").(*pg.DB)

	user.Type = "user"

	err = db.Create(&user)
	if err != nil {
		logger.Error(err.Error())
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	w.WriteHeader(http.StatusCreated)
	if err := json.NewEncoder(w).Encode(user); err != nil {
		logger.Error(err.Error())
		http.Error(w, err.Error(), http.StatusBadRequest)
	}
}

func RelationsInfoHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	user_id := vars["user_id"]
	db := context.Get(r, "db").(*pg.DB)

	var relationships []model.Relationship
	err := db.Model(&relationships).WhereOr(
		pg.SQL("from_id = ?", user_id),
		pg.SQL("to_id = ?", user_id),
	).Select()

	if err != nil {
		logger.Error(err.Error())
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(relationships); err != nil {
		logger.Error(err.Error())
		http.Error(w, err.Error(), http.StatusBadRequest)
	}
}

func UpdateRelationHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	params := model.Relationship{}
	params.From_id = vars["user_id"]
	params.To_id = vars["other_user_id"]
	decoder := json.NewDecoder(r.Body)
	err := decoder.Decode(&params)
	if err != nil {
		logger.Error(err.Error())
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	db := context.Get(r, "db").(*pg.DB)

	relationship := model.Relationship{}
	other_relationship := model.Relationship{}
	err = db.Model(&relationship).Where("from_id = ?", params.From_id).Where("to_id = ?", params.To_id).Select()
	if err != nil {
		logger.Error(err.Error())
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	err = db.Model(&other_relationship).Where("from_id = ?", params.To_id).Where("to_id = ?", params.From_id).Select()
	if err != nil {
		logger.Error(err.Error())
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	relationship.State = params.State

	if other_relationship.State == "matched" && relationship.State == "disliked" {
		other_relationship.State = "liked"
	} else if other_relationship.State == "liked" && relationship.State == "liked" {
		other_relationship.State = "matched"
		relationship.State = "matched"
	} else {

	}

	err = db.Update(&relationship)
	if err != nil {
		logger.Error(err.Error())
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	err = db.Update(&other_relationship)
	if err != nil {
		logger.Error(err.Error())
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	var relationships []*model.Relationship
	relationships = append(relationships, &relationship)
	relationships = append(relationships, &other_relationship)

	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(relationships); err != nil {
		logger.Error(err.Error())
		http.Error(w, err.Error(), http.StatusBadRequest)
	}
}

func localLog(fn http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		fn(w, r)
		logger.Info(
			r.Method,
			r.RequestURI,
			time.Since(start),
		)
	}
}

func main() {
	logger.SetRollingFile("./log", "tantan.log", 10, 5, logger.KB)
	logger.SetRollingDaily("./log", "tantan.log")
	logger.SetLevel(logger.DEBUG)

	dbpool, err := DBPool()
	if err != nil {
	}
	defer dbpool.Close()

	r := mux.NewRouter()
	r.HandleFunc("/", localLog(dbpool.WithData(HelloHandler))).Methods("GET")
	r.HandleFunc("/users", localLog(dbpool.WithData(UsersInfoHandler))).Methods("GET")
	r.HandleFunc("/users", localLog(dbpool.WithData(CreateUsersHandler))).Methods("POST")
	r.HandleFunc("/users/{user_id:[0-9]+}/relationships", localLog(dbpool.WithData(RelationsInfoHandler))).Methods("GET")
	r.HandleFunc("/users/{user_id:[0-9]+}/relationships/{other_user_id:[0-9]+}", localLog(dbpool.WithData(UpdateRelationHandler))).Methods("PUT")

	logger.Info("listen 127.0.0.1:8000")
	http.ListenAndServe(":8000", r)
}
