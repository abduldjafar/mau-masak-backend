# get customer by ID

{% swagger method="get" path="/v1/admin/customer/:id" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="path" name="id" type="cutomer id" required="true" %}

{% endswagger-parameter %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": {
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
