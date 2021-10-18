# Login

{% swagger baseUrl="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/login" method="post" summary="Users Login" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="body" name="body" type="object" %}

{% endswagger-parameter %}

{% swagger-response status="200" description="Cake successfully retrieved." %}
```javascript
{
    "responses": {
        "code": 200,
        "data": {
            "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNDk5ZTZiM2ZhNDA0NjIxNWQyODczNyIsImVtYWlsIjoiYWJkdWx4IiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzIzNjI1MTZ9.Jenrr6NnYBfj4lRkHNJjLiCjYzh1m8tpHLbi8ouwbEE"
        },
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
{% endswagger %}

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
| fieldname | type   | descriptipn      |
| --------- | ------ | ---------------- |
| email     | string | registered email |
| password  | string | users password   |
{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request POST 'https://mico-backend-services-i5jta7dz4q-uc.a.run.app/v1/login' \
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
