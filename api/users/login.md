# Login

{% api-method method="post" host="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/login" %}
{% api-method-summary %}
Users Login
{% endapi-method-summary %}

{% api-method-description %}
This endpoint allows you to get free cakes.
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-body-parameters %}
{% api-method-parameter name="body" type="object" required=true %}

{% endapi-method-parameter %}
{% endapi-method-body-parameters %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}
Cake successfully retrieved.
{% endapi-method-response-example-description %}

```javascript
{
    "responses": {
        "code": 200,
        "data": {
            "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNDk5ZTZiM2ZhNDA0NjIxNWQyODczNyIsImVtYWlsIjoiYWJkdWx4IiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzIzNjI1MTZ9.Jenrr6NnYBfj4lRkHNJjLiCjYzh1m8tpHLbi8ouwbEE",
            "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNDk5ZTZiM2ZhNDA0NjIxNWQyODczNyIsImVtYWlsIjoiYWJkdWx4IiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzI4ODA5MTZ9.n2AhDdQy6gtx2xHHBXDT_bHrSt2YH7y4i6edjmG9i5Y"
        },
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
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}

### Body Request

{% tabs %}
{% tab title="Json Body " %}
```javascript
{
    "email":"abdulx",
    "password":"assoi"
}
```
{% endtab %}

{% tab title="Description" %}
| fieldname | type | descriptipn |
| :--- | :--- | :--- |
| email | string | registered email |
| password | string | users password |
{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request POST 'http://localhost:8080/v1/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email":"abdulx",
    "password":"assoi"
}'
```
{% endtab %}

{% tab title="JavaScript - Fetch" %}
```javascript
var myHeaders = new Headers();
myHeaders.append("Content-Type", "application/json");

var raw = JSON.stringify({
  "email": "abdulx",
  "password": "assoi"
});

var requestOptions = {
  method: 'POST',
  headers: myHeaders,
  body: raw,
  redirect: 'follow'
};

fetch("http://localhost:8080/v1/login", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
{% endtab %}

{% tab title="JavaScript - JQuery" %}
```javascript
var myHeaders = new Headers();
myHeaders.append("Content-Type", "application/json");

var raw = JSON.stringify({
  "email": "abdulx",
  "password": "assoi"
});

var requestOptions = {
  method: 'POST',
  headers: myHeaders,
  body: raw,
  redirect: 'follow'
};

fetch("http://localhost:8080/v1/login", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
{% endtab %}
{% endtabs %}

