# User Get All Address

{% api-method method="get" host="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/users/address" %}
{% api-method-summary %}
user get all address
{% endapi-method-summary %}

{% api-method-description %}
This endpoint allows you to get free cakes.
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-headers %}
{% api-method-parameter name="Authorization" type="string" required=true %}

{% endapi-method-parameter %}
{% endapi-method-headers %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}
Cake successfully retrieved.
{% endapi-method-response-example-description %}

```
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "6155290af8e7be3a693ffaa7",
                "user_id": "6154d3810fd0274fa41e993c",
                "recipent_name": "aasoesoi",
                "country": "asoi",
                "phone_number": "asoi",
                "address_line1": "xxxx",
                "address_line2": "asoi",
                "city": "asoi",
                "state": "asoi",
                "postal_code": "asoi"
            },
            {
                "_id": "61552b5720002320474aac60",
                "user_id": "6154d3810fd0274fa41e993c",
                "recipent_name": "asoi",
                "country": "asoi",
                "phone_number": "asoi",
                "address_line1": "asoi",
                "address_line2": "asoi",
                "city": "asoi",
                "state": "asoi",
                "postal_code": "asoi"
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
Could not find a cake matching this query.
{% endapi-method-response-example-description %}

```
{
    "responses": {
        "code": 400,
        "data":null,
        "isError": true,
        "message": "message form server"
    }
}
```
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}



### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request GET 'http://localhost:8080/v1/users/address' \
--header 'Authorization: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiNXlvdXNlZnNhbG1hbmFAaXNlb3ZlbHMuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzMwNTA0MDl9.ISpxv_BTOuxQbI0nRdvUSiEW2v-XYBdOsAQpqvsalWo'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}

