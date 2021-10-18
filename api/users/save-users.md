# Save Users

{% swagger baseUrl="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/users" method="post" summary="save users" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="body" name="" type="object" %}
json body for create users. the bodies have explained in below
{% endswagger-parameter %}

{% swagger-response status="200" description="" %}
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

{% swagger-response status="401" description="" %}
```
{
    "code": 401,
    "data": null,
    "isError": true,
    "message": "token contains an invalid number of segments"
}
```
{% endswagger-response %}
{% endswagger %}

### Body Request

{% tabs %}
{% tab title="Json Body" %}
```
{
    "email":"koteka@gmail.com",
    "name":"koteka exchain",
    "password":"asoi geboi"
}
```
{% endtab %}

{% tab title="Descriptions" %}
| field name | type   | description             |
| ---------- | ------ | ----------------------- |
| email      | string | user email for register |
| name       | string | user name for register  |
| password   | string | user password for auth  |


{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```
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
```
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
