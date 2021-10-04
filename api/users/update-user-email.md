# Update User Email

{% api-method method="put" host="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/users/email\_profile" %}
{% api-method-summary %}
Get Cakes
{% endapi-method-summary %}

{% api-method-description %}
This endpoint allows you to get free cakes.
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-headers %}
{% api-method-parameter name="Authorization" type="string" required=true %}
Authentication token to track down who is emptying our stocks.
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
        "data": null,
        "isError": false,
        "message": "success"
    }
}
```
{% endapi-method-response-example %}

{% api-method-response-example httpCode=404 %}
{% api-method-response-example-description %}
Could not find a cake matching this query.
{% endapi-method-response-example-description %}

```
{
    "responses": {
        "code": 400,
        "data": null,
        "isError": true,
        "message": "email already registered"
    }
}
```
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}

## Body Request

{% tabs %}
{% tab title="Json Body" %}
```javascript
{
    "old_email":"5yousefsalmana@iseovels.com",
    "new_email":"xxx@iseovels.com"
    "password":"12345"
}
```
{% endtab %}

{% tab title="Description" %}
| field | type | description |
| :--- | :--- | :--- |
| old\_email | string | old email that want change |
| new\_email | string | new email for change |
| password | string | password for verification |
{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request PUT 'http://localhost:8080/v1/users/email_profile' \
--header 'Authorization: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiM3hpdXRhbC5taW5nY0ByaGVpb3AuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzMwNDA2MTN9.e_5rmTADMwbU2NBgruhzTdEhvzyddNRQrjoGHX11OCc' \
--header 'Content-Type: application/json' \
--data-raw '{
    "old_email":"5yousefsalmana@iseovels.com",
    "new_email":"xxx@iseovels.com"
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
    "old_email":"5yousefsalmana@iseovels.com",
    "new_email":"xxx@iseovels.com"
    "password":"12345"
});

var requestOptions = {
  method: 'PUT',
  headers: myHeaders,
  body: raw,
  redirect: 'follow'
};

fetch("http://localhost:8080/v1/users/email_profile", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
{% endtab %}

{% tab title="JavaScript - jQuery" %}
```javascript
var settings = {
  "url": "http://localhost:8080/v1/users/email_profile",
  "method": "PUT",
  "timeout": 0,
  "headers": {
    "Authorization": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiM3hpdXRhbC5taW5nY0ByaGVpb3AuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzMwNDA2MTN9.e_5rmTADMwbU2NBgruhzTdEhvzyddNRQrjoGHX11OCc",
    "Content-Type": "application/json"
  },
  "data": JSON.stringify({
    "old_email":"5yousefsalmana@iseovels.com",
    "new_email":"xxx@iseovels.com"
    "password":"12345"
}),
};

$.ajax(settings).done(function (response) {
  console.log(response);
});
```
{% endtab %}
{% endtabs %}

