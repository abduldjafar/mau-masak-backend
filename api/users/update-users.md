# Update Users Profile

{% swagger baseUrl="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/users/profile" method="put" summary="Update Users Profile" %}
{% swagger-description %}
This endpoint allows you to get free cakes.
{% endswagger-description %}

{% swagger-parameter in="header" name="Authentication" type="string" %}
Authentication token to track down who is emptying our stocks.
{% endswagger-parameter %}

{% swagger-parameter in="body" name="body" type="object" %}
all datas should be send and not empty
{% endswagger-parameter %}

{% swagger-response status="200" description="Cake successfully retrieved." %}
```javascript
{
    "responses": {
        "code": 200,
        "data": null,
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}

{% swagger-response status="400" description="Could not find a cake matching this query." %}
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

{% swagger-response status="401" description="" %}
```
{
    "responses": {
        "code": 401,
        "data": null,
        "isError": true,
        "message": "error message will produce by server"
    }
}
```
{% endswagger-response %}
{% endswagger %}

### Body Request

{% tabs %}
{% tab title="Json Bodys" %}
```javascript
{
    "name": "koteka",
    "birthdate": "2021-09-22T03:23:23.275+00:00",
    "gender":""

}
```
{% endtab %}

{% tab title="Description" %}
| fieldname | type   | description |
| --------- | ------ | ----------- |
| name      | string |             |
| birthdate | string |             |
| gender    | string |             |
|           |        |             |
{% endtab %}
{% endtabs %}
