# Save Users

{% api-method method="post" host="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/users" %}
{% api-method-summary %}
save users
{% endapi-method-summary %}

{% api-method-description %}

{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-body-parameters %}
{% api-method-parameter name="" type="object" required=true %}
json body for create users. the bodies have explained in below
{% endapi-method-parameter %}
{% endapi-method-body-parameters %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}

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

{% api-method-response-example httpCode=401 %}
{% api-method-response-example-description %}

{% endapi-method-response-example-description %}

```
{
    "code": 401,
    "data": null,
    "isError": true,
    "message": "token contains an invalid number of segments"
}
```
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}

### Body Request

{% tabs %}
{% tab title="Json Body" %}
```text
{
    "email":"koteka@gmail.com",
    "name":"koteka exchain",
    "password":"asoi geboi"
}
```
{% endtab %}

{% tab title="Descriptions" %}
| field name | type | description |
| :--- | :--- | :--- |
| email | string | user email for register |
| name | string | user name for register |
| password | string | user password for auth |
{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```text
curl --location --request POST 'https://api-dev.mico.sg/v1/users/save' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "koteka@gmail.com",
    "name": "koteka exchain",
    "password": "asoi geboi"
}'
```
{% endtab %}

{% tab title="JavaScript - Fetch" %}
```text
var myHeaders = new Headers();
myHeaders.append("Content-Type", "application/json");

var raw = JSON.stringify({
  "email": "koteka@gmail.com",
  "name": "koteka exchain",
  "password": "asoi geboi"
});

var requestOptions = {
  method: 'POST',
  headers: myHeaders,
  body: raw,
  redirect: 'follow'
};

fetch("https://api-dev.mico.sg/v1/users/save", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
{% endtab %}

{% tab title="JavaScript - jQuery" %}
```
var settings = {
  "url": "https://api-dev.mico.sg/v1/users/save",
  "method": "POST",
  "timeout": 0,
  "headers": {
    "Content-Type": "application/json"
  },
  "data": JSON.stringify({
    "email": "koteka@gmail.com",
    "name": "koteka exchain",
    "password": "asoi geboi"
  }),
};

$.ajax(settings).done(function (response) {
  console.log(response);
});
```
{% endtab %}
{% endtabs %}

