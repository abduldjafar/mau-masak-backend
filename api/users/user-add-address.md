# User Add Address

{% api-method method="post" host="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/users/address" %}
{% api-method-summary %}
User Add Address
{% endapi-method-summary %}

{% api-method-description %}

{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-headers %}
{% api-method-parameter name="Authorization" type="string" required=true %}
Authentication token to track down who is emptying our stocks.
{% endapi-method-parameter %}
{% endapi-method-headers %}

{% api-method-body-parameters %}
{% api-method-parameter name="body" type="object" required=false %}

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
        "message": "message from server"
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
    "recipent_name": "asoi",
    "country": "asoi",
    "phone_number": "asoi",
    "address_line1": "asoi",
    "address_line2": "asoi",
    "city": "asoi",
    "state": "asoi",
    "postal_code": "asoi"
}
```
{% endtab %}

{% tab title="Description" %}
| field | type | description |
| :--- | :--- | :--- |
| recipent\_name | string |  |
| country | string |  |
| phone\_number | string |  |
| address\_line1 | string |  |
| address\_line2 | string |  |
| city | string |  |
| state | string |  |
| postal\_code | string |  |
{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request POST 'http://localhost:8080/v1/users/address' \
--header 'Authorization: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiNXlvdXNlZnNhbG1hbmFAaXNlb3ZlbHMuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzMwNTA0MDl9.ISpxv_BTOuxQbI0nRdvUSiEW2v-XYBdOsAQpqvsalWo' \
--header 'Content-Type: application/json' \
--data-raw '{
    "recipent_name": "asoi",
    "country": "asoi",
    "phone_number": "asoi",
    "address_line1": "asoi",
    "address_line2": "asoi",
    "city": "asoi",
    "state": "asoi",
    "postal_code": "asoi"
}'
```
{% endtab %}

{% tab title="JavaScript - Fetch" %}
```javascript
var myHeaders = new Headers();
myHeaders.append("Authorization", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiNXlvdXNlZnNhbG1hbmFAaXNlb3ZlbHMuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzMwNTA0MDl9.ISpxv_BTOuxQbI0nRdvUSiEW2v-XYBdOsAQpqvsalWo");
myHeaders.append("Content-Type", "application/json");

var raw = JSON.stringify({
  "recipent_name": "asoi",
  "country": "asoi",
  "phone_number": "asoi",
  "address_line1": "asoi",
  "address_line2": "asoi",
  "city": "asoi",
  "state": "asoi",
  "postal_code": "asoi"
});

var requestOptions = {
  method: 'POST',
  headers: myHeaders,
  body: raw,
  redirect: 'follow'
};

fetch("http://localhost:8080/v1/users/address", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
{% endtab %}

{% tab title="JavaScript - jQuery" %}
```javascript
var settings = {
  "url": "http://localhost:8080/v1/users/address",
  "method": "POST",
  "timeout": 0,
  "headers": {
    "Authorization": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiNXlvdXNlZnNhbG1hbmFAaXNlb3ZlbHMuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzMwNTA0MDl9.ISpxv_BTOuxQbI0nRdvUSiEW2v-XYBdOsAQpqvsalWo",
    "Content-Type": "application/json"
  },
  "data": JSON.stringify({
    "recipent_name": "asoi",
    "country": "asoi",
    "phone_number": "asoi",
    "address_line1": "asoi",
    "address_line2": "asoi",
    "city": "asoi",
    "state": "asoi",
    "postal_code": "asoi"
  }),
};

$.ajax(settings).done(function (response) {
  console.log(response);
});
```
{% endtab %}
{% endtabs %}

