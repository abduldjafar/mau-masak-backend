# Get Users By ID

{% api-method method="get" host="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/users" %}
{% api-method-summary %}
get users by id
{% endapi-method-summary %}

{% api-method-description %}

{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-headers %}
{% api-method-parameter name="Authorization" type="string" required=true %}
token from login
{% endapi-method-parameter %}
{% endapi-method-headers %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}

{% endapi-method-response-example-description %}

```go
{
    "responses": {
        "code":200,
        "isError":false,
        "message":"success",
        "data":{
             "_id": "61482cecc7bff97f88829ad6",
                "name": "",
                "surname": "",
                "birthdate": "0001-01-01T00:00:00Z",
                "email": "abdul haris djafar",
                "mobile_number": "",
                "password": "",
                "address_street": "",
                "address_number": "",
                "address_postal_code": "",
                "address_country": "",
                "address_city": "",
                "verified": false,
                "type": "",
                "created_at": "0001-01-01T00:00:00Z"
        }
    }
}
```
{% endapi-method-response-example %}

{% api-method-response-example httpCode=302 %}
{% api-method-response-example-description %}

{% endapi-method-response-example-description %}

```

```
{% endapi-method-response-example %}

{% api-method-response-example httpCode=401 %}
{% api-method-response-example-description %}

{% endapi-method-response-example-description %}

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
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request GET 'http://localhost:8080/v1/users/:id' \
--header 'Authorization: xxxxxxxxxx'
```
{% endtab %}

{% tab title="JavaScript - Fetch" %}
```javascript
var myHeaders = new Headers();
myHeaders.append("Authorization", "xxxxxxxxxx");

var requestOptions = {
  method: 'GET',
  headers: myHeaders,
  redirect: 'follow'
};

fetch("http://localhost:8080/v1/users/:id", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
{% endtab %}

{% tab title="JavaScript - jQuery" %}
```javascript
var settings = {
  "url": "http://localhost:8080/v1/users/:id",
  "method": "GET",
  "timeout": 0,
  "headers": {
    "Authorization": "xxxxxxxxxx"
  },
};

$.ajax(settings).done(function (response) {
  console.log(response);
});
```
{% endtab %}
{% endtabs %}

