# muxpgtest

test restful api of GET, PUT, POST and DELETE with postgresql.

# Getting started

````bash
    (workspace) ➜  muxpgtest ./install
    (workspace) ➜  muxpgtest ./bin/server
    listen 127.0.0.1:8000
````

````bash
API TEST

(workspace) ➜  Downloads curl -XGET "http://localhost:8000/users"

(workspace) ➜  Downloads curl -XPOST -d '{"id": "11231244213", "name":"alice"}' "http://localhost:8000/users"
{
  "id": "11231244213",
  "name": "Alice",
  "type": "user"
}

(workspace) ➜  Downloads curl -XGET "http://localhost:8000/users/31231242322/relationships"

(workspace) ➜  Downloads curl -XPUT -d '{"state":"disliked"}' 'http://127.0.0.1:8000/users/31231242322/relationships/41231242323'
[{"Id":"444333222","From_id":"31231242322","To_id":"41231242323","State":"disliked","Type":"relationship"},{"Id":"555444333","From_id":"41231242323","To_id":"31231242322","State":"liked","Type":"relationship"}]

(workspace) ➜  curl -XPUT -d '{"state":"liked"}' 'http://127.0.0.1:8000/users/31231242322/relationships/41231242323'
[{"Id":"444333222","From_id":"31231242322","To_id":"41231242323","State":"matched","Type":"relationship"},{"Id":"555444333","From_id":"41231242323","To_id":"31231242322","State":"matched","Type":"relationship"}]
````