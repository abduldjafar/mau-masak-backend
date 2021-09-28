# Get All Vendors

{% api-method method="get" host="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/vendors/all" %}
{% api-method-summary %}
Get All Vendors
{% endapi-method-summary %}

{% api-method-description %}
This endpoint allows you to get free cakes.
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-headers %}
{% api-method-parameter name="Authorizarion" type="string" required=true %}
token from login
{% endapi-method-parameter %}
{% endapi-method-headers %}

{% api-method-query-parameters %}
{% api-method-parameter name="page" type="integer" %}
pagination from 0
{% endapi-method-parameter %}

{% api-method-parameter name="limit" type="integer" %}
limit of rows
{% endapi-method-parameter %}
{% endapi-method-query-parameters %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}

{% endapi-method-response-example-description %}

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
{% endapi-method-response-example %}

{% api-method-response-example httpCode=400 %}
{% api-method-response-example-description %}

{% endapi-method-response-example-description %}

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

