# User Validation

{% api-method method="post" host="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/users/validation" %}
{% api-method-summary %}
User Validation
{% endapi-method-summary %}

{% api-method-description %}
This endpoint allow you to validate user
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-headers %}
{% api-method-parameter name="Authorization" type="string" required=true %}
Authentication token to track down who is emptying our stocks.
{% endapi-method-parameter %}
{% endapi-method-headers %}

{% api-method-body-parameters %}
{% api-method-parameter name="object" type="object" required=true %}

{% endapi-method-parameter %}
{% endapi-method-body-parameters %}
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
        "data": null,
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
        "data": null,
        "isError": true,
        "message": "password mismatch"
    }
}
```
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}

### Body Request

{% tabs %}
{% tab title="Json Body" %}
```javascript
{
    "email":"3xiutal.mingc@rheiop.com",
    "password":"12345"
}
```
{% endtab %}

{% tab title="Description" %}
| field | type | description |
| :--- | :--- | :--- |
| email | string | user email |
| password | string | user password |
{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request POST 'http://localhost:8080/v1/users/validation' \
--header 'Authorization: eyJhbGciOiJIwEI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiM3hpdXRhbC5taW5nY0ByaGVpb3AuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzMwNDA2MTN9.e_5rmTADMwbU2NBgruhzTdEhvzyddNRQrjoGHX11OCc' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email":"3xiutal.mingc@rheiop.com",
    "password":"12345"
}'
```
{% endtab %}

{% tab title="JavaScript - Fetch" %}
```javascript
var myHeaders = new Headers();
myHeaders.append("Authorization", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiM3hpdXRhbC5taW5nY0ByaGVpb3AuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzMwNDA2MTN9.e_5rmTADMwbU2NBgruhzTdEhvzyddNRQrjoGHX11OCc");
myHeaders.append("Content-Type", "application/json");

var raw = JSON.stringify({
  "email": "3xiutal.mingc@rheiop.com",
  "password": "12345"
});

var requestOptions = {
  method: 'POST',
  headers: myHeaders,
  body: raw,
  redirect: 'follow'
};

fetch("http://localhost:8080/v1/users/validation", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
{% endtab %}

{% tab title="JavaScript - jQuery" %}
```javascript
var settings = {
  "url": "http://localhost:8080/v1/users/validation",
  "method": "POST",
  "timeout": 0,
  "headers": {
    "Authorization": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiM3hpdXRhbC5taW5nY0ByaGVpb3AuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzMwNDA2MTN9.e_5rmTADMwbU2NBgruhzTdEhvzyddNRQrjoGHX11OCc",
    "Content-Type": "application/json"
  },
  "data": JSON.stringify({
    "email": "3xiutal.mingc@rheiop.com",
    "password": "12345"
  }),
};

$.ajax(settings).done(function (response) {
  console.log(response);
});
```
{% endtab %}
{% endtabs %}

