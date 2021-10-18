# Update User Password

{% swagger baseUrl="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/users/password_profile" method="put" summary="Update User Password" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" type="string" %}
Authentication token to track down who is emptying our stocks.
{% endswagger-parameter %}

{% swagger-parameter in="body" name="object" type="object" %}

{% endswagger-parameter %}

{% swagger-response status="200" description="Cake successfully retrieved." %}
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
{% endswagger-response %}

{% swagger-response status="400" description="Could not find a cake matching this query." %}
```
{
    "responses": {
        "code": 400,
        "data": null,
        "isError": true,
        "message": "old password mismatch"
    }
}
```
{% endswagger-response %}
{% endswagger %}



### Body Requests

{% tabs %}
{% tab title="Json Body" %}
```javascript
{
    "new_password":"67890",
    "old_password":"12345"
}
```
{% endtab %}

{% tab title="Description" %}
| field        | type   | description                   |
| ------------ | ------ | ----------------------------- |
| old_password | string | old password that will change |
| new_password | string | new password for change       |
{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request PUT 'http://localhost:8080/v1/users/password_profile' \
--header 'Authorization: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiNXlvdXNlZnNhbG1hbmFAaXNlb3ZlbHMuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzMwNTAzMjh9.41gTMZCeAkslEyXfeOXXT8_XOnq5EHusvGEpZGGML2s' \
--header 'Content-Type: application/json' \
--data-raw '{
    "new_password":"67890",
    "old_password":"12345"
}'
```
{% endtab %}

{% tab title="JavaScript - Fetch" %}
```javascript
var myHeaders = new Headers();
myHeaders.append("Authorization", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiNXlvdXNlZnNhbG1hbmFAaXNlb3ZlbHMuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzMwNTAzMjh9.41gTMZCeAkslEyXfeOXXT8_XOnq5EHusvGEpZGGML2s");
myHeaders.append("Content-Type", "application/json");

var raw = JSON.stringify({
  "new_password": "67890",
  "old_password": "12345"
});

var requestOptions = {
  method: 'PUT',
  headers: myHeaders,
  body: raw,
  redirect: 'follow'
};

fetch("http://localhost:8080/v1/users/password_profile", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
{% endtab %}

{% tab title="JavaScript - jQuery" %}
```javascript
var settings = {
  "url": "http://localhost:8080/v1/users/password_profile",
  "method": "PUT",
  "timeout": 0,
  "headers": {
    "Authorization": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiNXlvdXNlZnNhbG1hbmFAaXNlb3ZlbHMuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzMwNTAzMjh9.41gTMZCeAkslEyXfeOXXT8_XOnq5EHusvGEpZGGML2s",
    "Content-Type": "application/json"
  },
  "data": JSON.stringify({
    "new_password": "67890",
    "old_password": "12345"
  }),
};

$.ajax(settings).done(function (response) {
  console.log(response);
});
```
{% endtab %}
{% endtabs %}
