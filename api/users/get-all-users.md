# Get All Users

{% api-method method="get" host="http://api-dev.mico.sg" path="/v1/users" %}
{% api-method-summary %}
Get All Users
{% endapi-method-summary %}

{% api-method-description %}
This endpoint allows you to get free cakes.
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-headers %}
{% api-method-parameter name="Authorization" type="string" required=true %}
token from login
{% endapi-method-parameter %}
{% endapi-method-headers %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}
Cake successfully retrieved.
{% endapi-method-response-example-description %}

```go
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "61482cecc7bff97f88829ad6",
                "name": "",
                "surname": "",
                "birthdate": "0001-01-01T00:00:00Z",
                "email": "abdul haris djafar",
                "mobile_number": "",
                "password": "",
                "address_street": "",
                "address_number": "",
                "address_postal_code": "",
                "address_country": "",
                "address_city": "",
                "verified": false,
                "type": "",
                "created_at": "0001-01-01T00:00:00Z"
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endapi-method-response-example %}

{% api-method-response-example httpCode=400 %}
{% api-method-response-example-description %}

{% endapi-method-response-example-description %}

```
{
    "responses": {
        "code": 400,
        "data": null,
        "isError": true,
        "message": "message will ouput from server"
    }
}
```
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}



