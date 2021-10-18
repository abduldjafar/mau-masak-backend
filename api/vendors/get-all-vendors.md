# Get All Vendors

{% swagger baseUrl="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/vendors/all" method="get" summary="Get All Vendors" %}
{% swagger-description %}
This endpoint allows you to get free cakes.
{% endswagger-description %}

{% swagger-parameter in="header" name="Authorizarion" type="string" %}
token from login
{% endswagger-parameter %}

{% swagger-parameter in="query" name="page" type="integer" %}
pagination from 0
{% endswagger-parameter %}

{% swagger-parameter in="query" name="limit" type="integer" %}
limit of rows
{% endswagger-parameter %}

{% swagger-response status="200" description="" %}
```
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "61518b5ba58751d91a354bc9",
                "brand_or_vendor": "",
                "HQ_country": "",
                "business_model": "",
                "phone_number": "",
                "email": "abdul.haris.djafar@gmail.com",
                "address": "",
                "owner_name": "",
                "representative_name": "",
                "official_register_name": "",
                "vat_number": "",
                "bank_account": "",
                "certified_b_corporation": "",
                "category_or_products": ""
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}

{% swagger-response status="400" description="" %}
```
{
    "responses": {
        "code": 400,
        "data": null,
        "isError": true,
        "message": "error message will produce by server"
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

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request GET 'https://mico-backend-services-i5jta7dz4q-uc.a.run.app/v1/vendors/all?page=0&limit=10'
```
{% endtab %}

{% tab title="JavaScript - Fetch" %}
```javascript
var requestOptions = {
  method: 'GET',
  redirect: 'follow'
};

fetch("https://mico-backend-services-i5jta7dz4q-uc.a.run.app/v1/vendors/all?page=0&limit=10", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
{% endtab %}

{% tab title="JavaScript - jQuery" %}
```javascript
var settings = {
  "url": "https://mico-backend-services-i5jta7dz4q-uc.a.run.app/v1/vendors/all?page=0&limit=10",
  "method": "GET",
  "timeout": 0,
};

$.ajax(settings).done(function (response) {
  console.log(response);
});
```
{% endtab %}
{% endtabs %}
