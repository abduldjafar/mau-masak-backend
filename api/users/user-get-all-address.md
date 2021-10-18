# User Get All Address

{% swagger baseUrl="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/users/address" method="get" summary="user get all address" %}
{% swagger-description %}
This endpoint allows you to get free cakes.
{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" type="string" %}

{% endswagger-parameter %}

{% swagger-response status="200" description="Cake successfully retrieved." %}
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
{% endswagger-response %}

{% swagger-response status="400" description="Could not find a cake matching this query." %}
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
{% endswagger-response %}
{% endswagger %}



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
