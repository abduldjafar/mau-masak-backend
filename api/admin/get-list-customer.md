# get list customer

{% swagger method="get" path="/v1/admin/customer" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="query" name="limit" required="false" type="int" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="page" type="int" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "61526e39cf7acd9b14bbc943",
                "stripe_id": "",
                "name": "",
                "gender": "",
                "birthdate": "0001-01-01T00:00:00Z",
                "email": "abdul.haris.djafar@gmail.com",
                "mobile_number": "",
                "password": "",
                "verified": false,
                "user_type": "",
                "photo_profile": "",
                "country_origin": "",
                "mico_score": {
                    "co2": 0,
                    "water": 0,
                    "non_recyle": false
                },
                "created_at": "2021-09-28T01:22:01.132Z"
            },
            {
                "_id": "61526e4fcf7acd9b14bbc945",
                "stripe_id": "",
                "name": "",
                "gender": "",
                "birthdate": "0001-01-01T00:00:00Z",
                "email": "abdul.haris.djafar@gmail.coms",
                "mobile_number": "",
                "password": "",
                "verified": false,
                "user_type": "",
                "photo_profile": "",
                "country_origin": "",
                "mico_score": {
                    "co2": 0,
                    "water": 0,
                    "non_recyle": false
                },
                "created_at": "2021-09-28T01:22:23.139Z"
            },
            {
                "_id": "61526e68cf7acd9b14bbc946",
                "stripe_id": "",
                "name": "Arbiyanto",
                "gender": "",
                "birthdate": "1999-09-01T17:00:00Z",
                "email": "user@testing.com",
                "mobile_number": "",
                "password": "",
                "verified": true,
                "user_type": "",
                "photo_profile": "",
                "country_origin": "",
                "mico_score": {
                    "co2": 0,
                    "water": 0,
                    "non_recyle": false
                },
                "created_at": "2021-09-28T01:22:48.145Z"
            },
            {
                "_id": "6153bb8d513267af8d121033",
                "stripe_id": "",
                "name": "asasas",
                "gender": "",
                "birthdate": "0001-01-01T00:00:00Z",
                "email": "ecomici1@gmail.com",
                "mobile_number": "",
                "password": "",
                "verified": false,
                "user_type": "",
                "photo_profile": "",
                "country_origin": "",
                "mico_score": {
                    "co2": 0,
                    "water": 0,
                    "non_recyle": false
                },
                "created_at": "2021-09-29T01:04:13.409Z"
            },
            {
                "_id": "6154944ff048c81762743947",
                "stripe_id": "",
                "name": "Testing Account",
                "gender": "",
                "birthdate": "0001-01-01T00:00:00Z",
                "email": "testing@yopmail.com",
                "mobile_number": "",
                "password": "",
                "verified": false,
                "user_type": "",
                "photo_profile": "",
                "country_origin": "",
                "mico_score": {
                    "co2": 0,
                    "water": 0,
                    "non_recyle": false
                },
                "created_at": "2021-09-29T16:29:03.226Z"
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}

{% swagger-response status="400: Bad Request" description="" %}
```javascript
{
    "responses": {
        "code": 400,
        "data": null,
        "isError": true,
        "message": "from server"
    }
}
```
{% endswagger-response %}
{% endswagger %}
