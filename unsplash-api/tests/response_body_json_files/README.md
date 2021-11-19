## Ok json files

| File                       | Url                                      |
| -------------------------- | ---------------------------------------- |
| list_photos_ok.json        | /photos/?client_id=xxx                   |
| search_photos_dogs_ok.json | /search/photos/?client_id=xxx&query=dogs |


## Err json files

| File                                 | Status | Case                         |
| ------------------------------------ | ------ | ---------------------------- |
| err_invalid_access_token.json        | 401    | /photos/?client_id=wrong     |
| err_search_photos_missing_query.json | 400    | /search/photos?client_id=xxx |
